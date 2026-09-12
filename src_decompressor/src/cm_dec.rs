//! Orpane Arithmetic Kernel — Pure Inverse Bit-Prediction Decoder
//! Decoding-only adaptive bit-predictor arithmetic decoder.

use std::sync::LazyLock;

const TOP: u64 = 1u64 << 32;

static SQUASH_LUT: LazyLock<[u16; 4096]> = LazyLock::new(|| {
    let mut lut = [0u16; 4096];
    for d in -2048..2048i32 {
        let x = d.clamp(-2047, 2047);
        let f = 1.0 / (1.0 + (-x as f64 / 400.0).exp());
        let val = ((f * 4095.0) as u32).clamp(1, 4094) as u16;
        lut[(d + 2048) as usize] = val;
    }
    lut
});

static STRETCH_LUT: LazyLock<[i16; 4096]> = LazyLock::new(|| {
    let mut lut = [0i16; 4096];
    for p in 0..4096u32 {
        let f = (p as f64 / 4095.0).clamp(1e-6, 1.0 - 1e-6);
        let val = (f.ln() / (1.0 - f).ln() * 400.0) as i32;
        lut[p as usize] = val.clamp(-2048, 2048) as i16;
    }
    lut
});

#[inline(always)]
fn squash(d: i32) -> u32 {
    let idx = (d.clamp(-2048, 2047) + 2048) as usize;
    SQUASH_LUT[idx] as u32
}

#[inline(always)]
fn stretch(p: u32) -> i32 {
    let idx = (p & 4095) as usize;
    STRETCH_LUT[idx] as i32
}

pub struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    cur_byte: u8,
    bits_left: u8,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0, cur_byte: 0, bits_left: 0 }
    }

    #[inline(always)]
    pub fn read_bit(&mut self) -> u8 {
        if self.bits_left == 0 {
            if self.pos < self.data.len() {
                self.cur_byte = self.data[self.pos];
                self.pos += 1;
                self.bits_left = 8;
            } else {
                return 0;
            }
        }
        self.bits_left -= 1;
        (self.cur_byte >> self.bits_left) & 1
    }
}

const APM_CONTEXTS: usize = 64;

pub struct AdaptiveProbabilityMap {
    table: Vec<[u16; 33]>,
}

impl AdaptiveProbabilityMap {
    pub fn new() -> Self {
        let mut table = vec![[0u16; 33]; APM_CONTEXTS];
        for ctx in 0..APM_CONTEXTS {
            for bin in 0..=32 {
                let p = ((bin as u32 * 4096 / 32) as u16).clamp(1, 4094);
                table[ctx][bin] = p;
            }
        }
        Self { table }
    }

    #[inline(always)]
    pub fn predict(&self, ctx: usize, p: u32) -> u32 {
        let c = ctx % APM_CONTEXTS;
        let p_clamped = p.clamp(1, 4094);
        let bin = (p_clamped / 128).min(31) as usize;
        let frac = p_clamped % 128;
        let v0 = self.table[c][bin] as u32;
        let v1 = self.table[c][bin + 1] as u32;
        ((v0 * (128 - frac) + v1 * frac) / 128).clamp(1, 4094)
    }

    #[inline(always)]
    pub fn update(&mut self, ctx: usize, p: u32, bit: u8) {
        let c = ctx % APM_CONTEXTS;
        let p_clamped = p.clamp(1, 4094);
        let bin = (p_clamped / 128).min(31) as usize;
        let frac = p_clamped % 128;
        let target = (bit as i32) * 4095;
        let pred = self.predict(c, p) as i32;
        let err = (target - pred) >> 5;

        let adj0 = err * (128 - frac as i32) / 128;
        let adj1 = err * (frac as i32) / 128;

        self.table[c][bin] = (self.table[c][bin] as i32 + adj0).clamp(1, 4094) as u16;
        self.table[c][bin + 1] = (self.table[c][bin + 1] as i32 + adj1).clamp(1, 4094) as u16;
    }
}

const TABLE_SIZE: usize = 65536;
const TABLE_MASK: usize = TABLE_SIZE - 1;

#[derive(Clone, Copy)]
struct Entry {
    tag: u32,
    c0: u16,
    c1: u16,
}

#[derive(Clone, Copy, Debug)]
pub enum ModelType {
    Order(usize),
    Sparse(usize),
    Word,
}

pub struct FlatModelTable {
    model_type: ModelType,
    entries: Vec<Entry>,
}

impl FlatModelTable {
    pub fn new(model_type: ModelType) -> Self {
        Self {
            model_type,
            entries: vec![Entry { tag: 0, c0: 0, c1: 0 }; TABLE_SIZE],
        }
    }

    #[inline(always)]
    fn hash_ctx(ctx: u64) -> usize {
        let h = ctx ^ (ctx >> 16) ^ (ctx >> 32);
        h.wrapping_mul(0x9E3779B9) as usize & TABLE_MASK
    }

    #[inline(always)]
    pub fn predict(&self, ctx: u64) -> u32 {
        let idx = Self::hash_ctx(ctx);
        let tag = (ctx >> 16) as u32 | 1;
        let e = &self.entries[idx];
        if e.tag == tag {
            let total = e.c0 as u32 + e.c1 as u32 + 2;
            ((e.c1 as u32 + 1) * 4094 / total).clamp(1, 4094)
        } else {
            2047
        }
    }

    #[inline(always)]
    pub fn update(&mut self, ctx: u64, bit: u8) {
        let idx = Self::hash_ctx(ctx);
        let tag = (ctx >> 16) as u32 | 1;
        let e = &mut self.entries[idx];
        if e.tag == tag {
            if bit == 0 {
                e.c0 = e.c0.saturating_add(1);
                if e.c0 == u16::MAX { e.c0 >>= 1; e.c1 >>= 1; }
            } else {
                e.c1 = e.c1.saturating_add(1);
                if e.c1 == u16::MAX { e.c0 >>= 1; e.c1 >>= 1; }
            }
        } else {
            e.tag = tag;
            e.c0 = (1 - bit) as u16;
            e.c1 = bit as u16;
        }
    }

    #[inline(always)]
    pub fn context_key(&self, history: u64) -> u64 {
        match self.model_type {
            ModelType::Order(o) => {
                let mask = if o >= 8 { !0u64 } else { (1u64 << (o * 8)) - 1 };
                history & mask
            }
            ModelType::Sparse(stride) => {
                let latest = history & 0xFF;
                let strided = (history >> (stride * 8)) & 0xFF;
                (strided << 8) | latest
            }
            ModelType::Word => {
                let b0 = (history & 0xFF) as u8;
                let b1 = ((history >> 8) & 0xFF) as u8;
                let u0 = b0.to_ascii_uppercase() as u64;
                let u1 = b1.to_ascii_uppercase() as u64;
                (u1 << 8) | u0
            }
        }
    }
}

pub struct MatchModel {
    history_buf: Vec<u8>,
    hash_tab: Vec<i32>,
    match_pos: usize,
    match_len: usize,
}

impl MatchModel {
    pub fn new() -> Self {
        Self {
            history_buf: Vec::with_capacity(65536),
            hash_tab: vec![-1; 65536],
            match_pos: 0,
            match_len: 0,
        }
    }

    #[inline(always)]
    fn hash4(b: &[u8]) -> usize {
        let v = (b[0] as usize) | ((b[1] as usize) << 8) | ((b[2] as usize) << 16) | ((b[3] as usize) << 24);
        (v.wrapping_mul(0x9E3779B9) >> 16) & 0xFFFF
    }

    #[inline(always)]
    pub fn on_byte_boundary(&mut self) {
        let n = self.history_buf.len();
        if n >= 4 {
            let h = Self::hash4(&self.history_buf[n - 4..n]);
            let prev = self.hash_tab[h];
            self.hash_tab[h] = (n - 4) as i32;
            if prev >= 0 {
                let p = prev as usize;
                if p + 4 < n && self.history_buf[p..p + 4] == self.history_buf[n - 4..n] {
                    self.match_pos = p + 4;
                    self.match_len = 4;
                    return;
                }
            }
        }
        self.match_len = 0;
    }

    #[inline(always)]
    pub fn predict_bit(&self, bit_pos: usize) -> u32 {
        if self.match_len >= 4 && self.match_pos < self.history_buf.len() {
            let expected_byte = self.history_buf[self.match_pos];
            let expected_bit = (expected_byte >> bit_pos) & 1;
            if expected_bit == 1 { 4000 } else { 96 }
        } else {
            2047
        }
    }

    #[inline(always)]
    pub fn push_byte(&mut self, actual_byte: u8) {
        if self.match_len >= 4 {
            if self.match_pos < self.history_buf.len() && actual_byte == self.history_buf[self.match_pos] {
                self.match_pos += 1;
                self.match_len += 1;
            } else {
                self.match_len = 0;
            }
        }
        self.history_buf.push(actual_byte);
    }
}

pub struct IndirectContextModel {
    last_symbol: Vec<u8>,
    sse_counts: Vec<[u16; 2]>,
}

impl IndirectContextModel {
    pub fn new() -> Self {
        Self {
            last_symbol: vec![0u8; 65536],
            sse_counts: vec![[1u16, 1u16]; 65536],
        }
    }

    #[inline(always)]
    fn hash_ctx(history: u64) -> usize {
        let h = (history & 0xFFFF_FFFF) ^ (history >> 16) ^ (history >> 32);
        h.wrapping_mul(0x9E3779B9) as usize & 0xFFFF
    }

    #[inline(always)]
    pub fn predict(&self, history: u64, bit_pos: usize) -> u32 {
        let h = Self::hash_ctx(history);
        let predicted_byte = self.last_symbol[h] as usize;
        let partial_prefix = ((history & 0x7F) as usize) | ((7 - bit_pos) << 7);
        let sse_idx = ((predicted_byte << 8) | (partial_prefix & 0xFF)) & 0xFFFF;
        let c0 = self.sse_counts[sse_idx][0] as u32;
        let c1 = self.sse_counts[sse_idx][1] as u32;
        let total = c0 + c1;
        ((c1 * 4094) / total).clamp(1, 4094)
    }

    #[inline(always)]
    pub fn update(&mut self, history: u64, bit_pos: usize, bit: u8) {
        let h = Self::hash_ctx(history);
        let predicted_byte = self.last_symbol[h] as usize;
        let partial_prefix = ((history & 0x7F) as usize) | ((7 - bit_pos) << 7);
        let sse_idx = ((predicted_byte << 8) | (partial_prefix & 0xFF)) & 0xFFFF;
        if bit == 0 {
            self.sse_counts[sse_idx][0] = self.sse_counts[sse_idx][0].saturating_add(1);
            if self.sse_counts[sse_idx][0] == u16::MAX {
                self.sse_counts[sse_idx][0] >>= 1;
                self.sse_counts[sse_idx][1] >>= 1;
            }
        } else {
            self.sse_counts[sse_idx][1] = self.sse_counts[sse_idx][1].saturating_add(1);
            if self.sse_counts[sse_idx][1] == u16::MAX {
                self.sse_counts[sse_idx][0] >>= 1;
                self.sse_counts[sse_idx][1] >>= 1;
            }
        }
    }

    #[inline(always)]
    pub fn push_byte(&mut self, history: u64, actual_byte: u8) {
        let h = Self::hash_ctx(history);
        self.last_symbol[h] = actual_byte;
    }
}

pub struct Mixer {
    pub weights: Vec<i32>,
    pub eta: i32,
}

impl Mixer {
    pub fn new(n: usize) -> Self {
        Self { weights: vec![256i32; n], eta: 8 }
    }

    #[inline(always)]
    pub fn predict(&self, preds: &[u32]) -> u32 {
        let mut sum = 0i64;
        for (i, &p) in preds.iter().enumerate() {
            sum += self.weights[i] as i64 * stretch(p) as i64;
        }
        squash((sum / 1024).clamp(-2047, 2047) as i32)
    }

    #[inline(always)]
    pub fn update(&mut self, preds: &[u32], bit: u8, p_mixed: u32) {
        let err = bit as i32 * 4095 - p_mixed as i32;
        for (i, &p) in preds.iter().enumerate() {
            self.weights[i] += (self.eta as i64 * err as i64 * stretch(p) as i64 / (1024 * 1024)) as i32;
            self.weights[i] = self.weights[i].clamp(-2048, 2048);
        }
    }
}

pub struct ArithDecoder<'a> {
    low: u64,
    high: u64,
    code: u64,
    reader: BitReader<'a>,
}

impl<'a> ArithDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let mut reader = BitReader::new(data);
        let mut code = 0u64;
        for _ in 0..32 {
            code = (code << 1) | reader.read_bit() as u64;
        }
        Self { low: 0, high: TOP - 1, code, reader }
    }

    #[inline(always)]
    pub fn decode_bit(&mut self, p1: u32) -> u8 {
        let range = self.high - self.low + 1;
        let p0 = 4096 - p1 as u64;
        let split = self.low + (range * p0) / 4096;
        let bit = if self.code <= split { 0u8 } else { 1u8 };
        if bit == 0 { self.high = split; } else { self.low = split + 1; }
        loop {
            if self.high < TOP / 2 {
                // bit 0 region
            } else if self.low >= TOP / 2 {
                self.low -= TOP / 2;
                self.high -= TOP / 2;
                self.code -= TOP / 2;
            } else if self.low >= TOP / 4 && self.high < 3 * TOP / 4 {
                self.low -= TOP / 4;
                self.high -= TOP / 4;
                self.code -= TOP / 4;
            } else { break; }
            self.low *= 2;
            self.high = self.high * 2 + 1;
            self.code = (self.code << 1) | self.reader.read_bit() as u64;
        }
        bit
    }
}

pub const MODEL_SPECS: &[ModelType] = &[
    ModelType::Order(1),
    ModelType::Order(2),
    ModelType::Order(4),
    ModelType::Order(8),
    ModelType::Sparse(2),
    ModelType::Sparse(4),
    ModelType::Word,
];

/// Context Mixing Bit Predictor Decode — Exact bit-for-bit inverse.
pub fn cm_decode(data: &[u8]) -> Result<Vec<u8>, &'static str> {
    if data.len() < 8 { return Err("CM: truncated header"); }
    let orig_len = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
    let packed_len = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;
    if data.len() < 8 + packed_len { return Err("CM: truncated bitstream"); }
    let packed = &data[8..8 + packed_len];

    let mut models: Vec<FlatModelTable> = MODEL_SPECS.iter().map(|&m| FlatModelTable::new(m)).collect();
    let mut match_model = MatchModel::new();
    let mut indirect_model = IndirectContextModel::new();
    let num_models = MODEL_SPECS.len() + 2;
    let mut mixer = Mixer::new(num_models);
    let mut apm = AdaptiveProbabilityMap::new();
    let mut dec = ArithDecoder::new(packed);
    let mut history = 0u64;
    let mut out = Vec::with_capacity(orig_len);

    for _ in 0..orig_len {
        match_model.on_byte_boundary();
        let mut byte = 0u8;
        for bit_pos in (0..8).rev() {
            let mut preds = [0u32; 16];
            for (i, m) in models.iter().enumerate() {
                preds[i] = m.predict(m.context_key(history));
            }
            preds[models.len()] = match_model.predict_bit(bit_pos);
            preds[models.len() + 1] = indirect_model.predict(history, bit_pos);

            let p_mixed = mixer.predict(&preds[..num_models]);
            let apm_ctx = ((history & 0x1F) as usize) | (bit_pos << 5);
            let p_final = apm.predict(apm_ctx, p_mixed);

            let bit = dec.decode_bit(p_final);
            byte |= bit << bit_pos;

            for m in models.iter_mut() {
                let k = m.context_key(history);
                m.update(k, bit);
            }
            indirect_model.update(history, bit_pos, bit);
            mixer.update(&preds[..num_models], bit, p_mixed);
            apm.update(apm_ctx, p_mixed, bit);

            history = (history << 1) | bit as u64;
        }
        match_model.push_byte(byte);
        indirect_model.push_byte(history, byte);
        out.push(byte);
    }
    Ok(out)
}

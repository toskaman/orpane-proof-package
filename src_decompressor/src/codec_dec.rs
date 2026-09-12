//! Orpane-LZ Pure Inverse Decoder
//! Standalone decompression pipeline (rANS, rANS-8, Tri-Stream Sequences).

pub const MAGIC_OLZ1: &[u8; 4] = b"OLZ1";
pub const MAGIC_OLZ2: &[u8; 4] = b"OLZ2";

const RANS_SCALE_BITS: u32 = 12;
const RANS_SCALE: u32 = 1 << RANS_SCALE_BITS; // 4096
const RANS_L: u32 = 1 << 15; // 32768

fn decode_leb128(buf: &[u8], mut pos: usize) -> Result<(u64, usize), String> {
    let mut val: u64 = 0;
    let mut shift = 0;
    while pos < buf.len() {
        let b = buf[pos];
        pos += 1;
        val |= ((b & 0x7F) as u64) << shift;
        if (b & 0x80) == 0 {
            return Ok((val, pos));
        }
        shift += 7;
        if shift >= 64 {
            return Err("LEB128 overflow".into());
        }
    }
    Err("Truncated LEB128".into())
}

#[derive(Debug, Clone)]
pub struct Sequence {
    pub lit_len: u32,
    pub match_len: u32,
    pub offset: u32,
}

pub struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    cur_byte: u8,
    bits_left: u8,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            cur_byte: 0,
            bits_left: 0,
        }
    }

    pub fn read_bits(&mut self, mut num_bits: usize) -> Result<u32, String> {
        if num_bits == 0 {
            return Ok(0);
        }
        let mut result = 0u32;
        let mut shift = 0usize;

        while num_bits > 0 {
            if self.bits_left == 0 {
                if self.pos >= self.data.len() {
                    return Err("Unexpected EOF in bit reader".into());
                }
                self.cur_byte = self.data[self.pos];
                self.pos += 1;
                self.bits_left = 8;
            }

            let take = self.bits_left.min(num_bits as u8);
            let mask = (1u32 << take) - 1;
            let bits = (self.cur_byte as u32) & mask;
            if shift < 32 {
                result |= bits << shift;
            }

            if take >= 8 {
                self.cur_byte = 0;
            } else {
                self.cur_byte >>= take;
            }
            self.bits_left -= take;
            shift += take as usize;
            num_bits -= take as usize;
        }

        Ok(result)
    }
}

pub struct RansCoder;

impl RansCoder {
    pub fn decode(payload: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
        if orig_len == 0 {
            return Ok(Vec::new());
        }
        if payload.is_empty() {
            return Err("Empty rANS payload".into());
        }

        let mut pos = 0;
        let nonzero = if payload[pos] == 0 { 256 } else { payload[pos] as usize };
        pos += 1;

        let mut norm_freqs = [0u32; 256];
        for _ in 0..nonzero {
            if pos + 3 > payload.len() {
                return Err("Truncated frequency table".into());
            }
            let idx = payload[pos] as usize;
            let nf = (payload[pos + 1] as u32) | ((payload[pos + 2] as u32) << 8);
            norm_freqs[idx] = nf;
            pos += 3;
        }

        let mut sum_freqs = 0u32;
        for &nf in &norm_freqs {
            sum_freqs += nf;
        }
        if sum_freqs != RANS_SCALE {
            return Err(format!("Invalid rANS frequency table sum: {}, expected {}", sum_freqs, RANS_SCALE));
        }

        let mut cum_freqs = [0u32; 257];
        for j in 0..256 {
            cum_freqs[j + 1] = cum_freqs[j] + norm_freqs[j];
        }

        #[derive(Clone, Copy)]
        #[repr(C, align(8))]
        struct DecEntry {
            sym: u8,
            _pad: u8,
            freq: u16,
            bias: u16,
            _pad2: u16,
        }

        let mut lut = [DecEntry { sym: 0, _pad: 0, freq: 1, bias: 0, _pad2: 0 }; RANS_SCALE as usize];

        for j in 0..256u32 {
            let f = norm_freqs[j as usize];
            if f == 0 { continue; }
            let c = cum_freqs[j as usize];
            for k in c..c + f {
                lut[k as usize] = DecEntry {
                    sym: j as u8,
                    _pad: 0,
                    freq: f as u16,
                    bias: c as u16,
                    _pad2: 0,
                };
            }
        }

        if pos + 4 > payload.len() {
            return Err("Truncated rANS state".into());
        }

        let mut state = u32::from_le_bytes(payload[pos..pos + 4].try_into().map_err(|e| format!("{:?}", e))?);
        pos += 4;

        let mut out = Vec::with_capacity(orig_len);
        let end = payload.len();

        for _ in 0..orig_len {
            let slot = (state & (RANS_SCALE - 1)) as usize;
            let ds = unsafe { *lut.get_unchecked(slot) };
            out.push(ds.sym);

            state = (ds.freq as u32) * (state >> RANS_SCALE_BITS) + (slot as u32) - (ds.bias as u32);

            if state < RANS_L {
                if pos < end {
                    state = (state << 8) | unsafe { *payload.get_unchecked(pos) as u32 };
                    pos += 1;
                }
                if state < RANS_L && pos < end {
                    state = (state << 8) | unsafe { *payload.get_unchecked(pos) as u32 };
                    pos += 1;
                }
            }
        }

        Ok(out)
    }
}

pub struct Rans8Coder;

impl Rans8Coder {
    pub fn decode(payload: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
        if orig_len == 0 {
            return Ok(Vec::new());
        }
        if payload.is_empty() {
            return Err("Empty rANS8 payload".into());
        }
        if payload[0] == 0 {
            // Mode 0: scalar fallback
            return RansCoder::decode(&payload[1..], orig_len);
        }

        let mut pos = 1; // skip mode byte
        if pos >= payload.len() {
            return Err("Truncated rANS8 stream".into());
        }

        let nonzero = if payload[pos] == 0 { 256 } else { payload[pos] as usize };
        pos += 1;

        let mut norm_freqs = [0u32; 256];
        for _ in 0..nonzero {
            if pos + 3 > payload.len() {
                return Err("Truncated frequency table in rANS8".into());
            }
            let idx = payload[pos] as usize;
            let nf = (payload[pos + 1] as u32) | ((payload[pos + 2] as u32) << 8);
            norm_freqs[idx] = nf;
            pos += 3;
        }

        let mut sum_freqs = 0u32;
        for &nf in &norm_freqs {
            sum_freqs += nf;
        }
        if sum_freqs != RANS_SCALE {
            return Err(format!("Invalid rANS8 freq table sum: {}, expected {}", sum_freqs, RANS_SCALE));
        }

        let mut cum_freqs = [0u32; 257];
        for j in 0..256 {
            cum_freqs[j + 1] = cum_freqs[j] + norm_freqs[j];
        }

        #[derive(Clone, Copy)]
        #[repr(C, align(8))]
        struct DecEntry {
            sym: u8,
            _pad: u8,
            freq: u16,
            bias: u16,
            _pad2: u16,
        }

        let mut lut = [DecEntry { sym: 0, _pad: 0, freq: 1, bias: 0, _pad2: 0 }; RANS_SCALE as usize];
        for j in 0..256u32 {
            let f = norm_freqs[j as usize];
            if f == 0 { continue; }
            let c = cum_freqs[j as usize];
            for k in c..c + f {
                lut[k as usize] = DecEntry {
                    sym: j as u8,
                    _pad: 0,
                    freq: f as u16,
                    bias: c as u16,
                    _pad2: 0,
                };
            }
        }

        if pos + 32 > payload.len() {
            return Err("Truncated stream lengths in rANS8".into());
        }
        let mut stream_lens = [0usize; 8];
        for k in 0..8 {
            let slen = u32::from_le_bytes(payload[pos..pos + 4].try_into().unwrap()) as usize;
            stream_lens[k] = slen;
            pos += 4;
        }

        let mut slices: [&[u8]; 8] = [&[]; 8];
        for k in 0..8 {
            let slen = stream_lens[k];
            if pos + slen > payload.len() {
                return Err(format!("Stream {} overflows rANS8 payload", k));
            }
            slices[k] = &payload[pos..pos + slen];
            pos += slen;
        }

        let mut states = [RANS_L; 8];
        let mut stream_pos = [0usize; 8];

        for k in 0..8 {
            let n_k = (orig_len / 8) + if k < (orig_len % 8) { 1 } else { 0 };
            if n_k > 0 {
                if slices[k].len() < 4 {
                    return Err(format!("Stream {} too short for initial state", k));
                }
                states[k] = u32::from_le_bytes(slices[k][0..4].try_into().unwrap());
                stream_pos[k] = 4;
            }
        }

        let mut out = Vec::with_capacity(orig_len);
        let chunks = orig_len / 8;

        let s0 = slices[0]; let s1 = slices[1]; let s2 = slices[2]; let s3 = slices[3];
        let s4 = slices[4]; let s5 = slices[5]; let s6 = slices[6]; let s7 = slices[7];
        let mut p0 = stream_pos[0]; let mut p1 = stream_pos[1];
        let mut p2 = stream_pos[2]; let mut p3 = stream_pos[3];
        let mut p4 = stream_pos[4]; let mut p5 = stream_pos[5];
        let mut p6 = stream_pos[6]; let mut p7 = stream_pos[7];
        let end0 = s0.len(); let end1 = s1.len(); let end2 = s2.len(); let end3 = s3.len();
        let end4 = s4.len(); let end5 = s5.len(); let end6 = s6.len(); let end7 = s7.len();

        let mut state0 = states[0]; let mut state1 = states[1];
        let mut state2 = states[2]; let mut state3 = states[3];
        let mut state4 = states[4]; let mut state5 = states[5];
        let mut state6 = states[6]; let mut state7 = states[7];

        for _ in 0..chunks {
            let slot0 = (state0 & (RANS_SCALE - 1)) as usize;
            let slot1 = (state1 & (RANS_SCALE - 1)) as usize;
            let slot2 = (state2 & (RANS_SCALE - 1)) as usize;
            let slot3 = (state3 & (RANS_SCALE - 1)) as usize;
            let slot4 = (state4 & (RANS_SCALE - 1)) as usize;
            let slot5 = (state5 & (RANS_SCALE - 1)) as usize;
            let slot6 = (state6 & (RANS_SCALE - 1)) as usize;
            let slot7 = (state7 & (RANS_SCALE - 1)) as usize;

            let e0 = unsafe { *lut.get_unchecked(slot0) };
            let e1 = unsafe { *lut.get_unchecked(slot1) };
            let e2 = unsafe { *lut.get_unchecked(slot2) };
            let e3 = unsafe { *lut.get_unchecked(slot3) };
            let e4 = unsafe { *lut.get_unchecked(slot4) };
            let e5 = unsafe { *lut.get_unchecked(slot5) };
            let e6 = unsafe { *lut.get_unchecked(slot6) };
            let e7 = unsafe { *lut.get_unchecked(slot7) };

            out.push(e0.sym);
            out.push(e1.sym);
            out.push(e2.sym);
            out.push(e3.sym);
            out.push(e4.sym);
            out.push(e5.sym);
            out.push(e6.sym);
            out.push(e7.sym);

            state0 = (e0.freq as u32) * (state0 >> RANS_SCALE_BITS) + (slot0 as u32) - (e0.bias as u32);
            state1 = (e1.freq as u32) * (state1 >> RANS_SCALE_BITS) + (slot1 as u32) - (e1.bias as u32);
            state2 = (e2.freq as u32) * (state2 >> RANS_SCALE_BITS) + (slot2 as u32) - (e2.bias as u32);
            state3 = (e3.freq as u32) * (state3 >> RANS_SCALE_BITS) + (slot3 as u32) - (e3.bias as u32);
            state4 = (e4.freq as u32) * (state4 >> RANS_SCALE_BITS) + (slot4 as u32) - (e4.bias as u32);
            state5 = (e5.freq as u32) * (state5 >> RANS_SCALE_BITS) + (slot5 as u32) - (e5.bias as u32);
            state6 = (e6.freq as u32) * (state6 >> RANS_SCALE_BITS) + (slot6 as u32) - (e6.bias as u32);
            state7 = (e7.freq as u32) * (state7 >> RANS_SCALE_BITS) + (slot7 as u32) - (e7.bias as u32);

            if state0 < RANS_L {
                if p0 < end0 { state0 = (state0 << 8) | unsafe { *s0.get_unchecked(p0) as u32 }; p0 += 1; }
                if state0 < RANS_L && p0 < end0 { state0 = (state0 << 8) | unsafe { *s0.get_unchecked(p0) as u32 }; p0 += 1; }
            }
            if state1 < RANS_L {
                if p1 < end1 { state1 = (state1 << 8) | unsafe { *s1.get_unchecked(p1) as u32 }; p1 += 1; }
                if state1 < RANS_L && p1 < end1 { state1 = (state1 << 8) | unsafe { *s1.get_unchecked(p1) as u32 }; p1 += 1; }
            }
            if state2 < RANS_L {
                if p2 < end2 { state2 = (state2 << 8) | unsafe { *s2.get_unchecked(p2) as u32 }; p2 += 1; }
                if state2 < RANS_L && p2 < end2 { state2 = (state2 << 8) | unsafe { *s2.get_unchecked(p2) as u32 }; p2 += 1; }
            }
            if state3 < RANS_L {
                if p3 < end3 { state3 = (state3 << 8) | unsafe { *s3.get_unchecked(p3) as u32 }; p3 += 1; }
                if state3 < RANS_L && p3 < end3 { state3 = (state3 << 8) | unsafe { *s3.get_unchecked(p3) as u32 }; p3 += 1; }
            }
            if state4 < RANS_L {
                if p4 < end4 { state4 = (state4 << 8) | unsafe { *s4.get_unchecked(p4) as u32 }; p4 += 1; }
                if state4 < RANS_L && p4 < end4 { state4 = (state4 << 8) | unsafe { *s4.get_unchecked(p4) as u32 }; p4 += 1; }
            }
            if state5 < RANS_L {
                if p5 < end5 { state5 = (state5 << 8) | unsafe { *s5.get_unchecked(p5) as u32 }; p5 += 1; }
                if state5 < RANS_L && p5 < end5 { state5 = (state5 << 8) | unsafe { *s5.get_unchecked(p5) as u32 }; p5 += 1; }
            }
            if state6 < RANS_L {
                if p6 < end6 { state6 = (state6 << 8) | unsafe { *s6.get_unchecked(p6) as u32 }; p6 += 1; }
                if state6 < RANS_L && p6 < end6 { state6 = (state6 << 8) | unsafe { *s6.get_unchecked(p6) as u32 }; p6 += 1; }
            }
            if state7 < RANS_L {
                if p7 < end7 { state7 = (state7 << 8) | unsafe { *s7.get_unchecked(p7) as u32 }; p7 += 1; }
                if state7 < RANS_L && p7 < end7 { state7 = (state7 << 8) | unsafe { *s7.get_unchecked(p7) as u32 }; p7 += 1; }
            }
        }

        let rem = orig_len % 8;
        states[0] = state0; stream_pos[0] = p0;
        states[1] = state1; stream_pos[1] = p1;
        states[2] = state2; stream_pos[2] = p2;
        states[3] = state3; stream_pos[3] = p3;
        states[4] = state4; stream_pos[4] = p4;
        states[5] = state5; stream_pos[5] = p5;
        states[6] = state6; stream_pos[6] = p6;
        states[7] = state7; stream_pos[7] = p7;

        for k in 0..rem {
            let mut st = states[k];
            let mut sp = stream_pos[k];
            let sk = slices[k];
            let end_k = sk.len();

            let slot = (st & (RANS_SCALE - 1)) as usize;
            let entry = unsafe { *lut.get_unchecked(slot) };
            out.push(entry.sym);

            st = (entry.freq as u32) * (st >> RANS_SCALE_BITS) + (slot as u32) - (entry.bias as u32);
            if st < RANS_L {
                if sp < end_k { st = (st << 8) | unsafe { *sk.get_unchecked(sp) as u32 }; sp += 1; }
                if st < RANS_L && sp < end_k { st = (st << 8) | unsafe { *sk.get_unchecked(sp) as u32 }; sp += 1; }
            }
            states[k] = st;
            stream_pos[k] = sp;
        }

        Ok(out)
    }
}

pub struct SequenceCoder;

impl SequenceCoder {
    pub fn decode(payload: &[u8]) -> Result<Vec<Sequence>, String> {
        if payload.is_empty() {
            return Ok(Vec::new());
        }

        let (num_seqs, mut off) = decode_leb128(payload, 0)?;
        let num_seqs = num_seqs as usize;
        if num_seqs > 500_000_000 {
            return Err("Excessive sequence count in stream".into());
        }
        if off >= payload.len() {
            return Err("Truncated sequence header".into());
        }

        let modes = payload[off];
        off += 1;
        let lit_mode = modes & 1;
        let match_mode = (modes >> 1) & 1;
        let offset_mode = (modes >> 2) & 1;

        let (lit_comp_len, new_off) = decode_leb128(payload, off)?; off = new_off;
        let lit_comp_len = lit_comp_len as usize;
        if off + lit_comp_len > payload.len() { return Err("Truncated lit_lens".into()); }
        let lit_slice = &payload[off..off + lit_comp_len]; off += lit_comp_len;
        let lit_lens = if lit_mode == 1 {
            RansCoder::decode(lit_slice, num_seqs)?
        } else {
            lit_slice.to_vec()
        };
        if lit_lens.len() < num_seqs {
            return Err(format!("Malformed sequence stream: lit_lens count ({}) < num_seqs ({})", lit_lens.len(), num_seqs));
        }

        let (match_comp_len, new_off) = decode_leb128(payload, off)?; off = new_off;
        let match_comp_len = match_comp_len as usize;
        if off + match_comp_len > payload.len() { return Err("Truncated match_lens".into()); }
        let match_slice = &payload[off..off + match_comp_len]; off += match_comp_len;
        let match_lens = if match_mode == 1 {
            RansCoder::decode(match_slice, num_seqs)?
        } else {
            match_slice.to_vec()
        };
        if match_lens.len() < num_seqs {
            return Err(format!("Malformed sequence stream: match_lens count ({}) < num_seqs ({})", match_lens.len(), num_seqs));
        }

        let (off_comp_len, new_off) = decode_leb128(payload, off)?; off = new_off;
        let off_comp_len = off_comp_len as usize;
        if off + off_comp_len > payload.len() { return Err("Truncated offset_codes".into()); }
        let off_slice = &payload[off..off + off_comp_len]; off += off_comp_len;

        let num_matches = match_lens[..num_seqs].iter().filter(|&&m| m > 0).count();
        let offset_codes = if offset_mode == 1 {
            RansCoder::decode(off_slice, num_matches)?
        } else {
            off_slice.to_vec()
        };
        if offset_codes.len() < num_matches {
            return Err(format!("Malformed sequence stream: offset_codes count ({}) < num_matches ({})", offset_codes.len(), num_matches));
        }

        let (extra_leb_len, new_off) = decode_leb128(payload, off)?; off = new_off;
        let extra_leb_len = extra_leb_len as usize;
        if off + extra_leb_len > payload.len() { return Err("Truncated extra_leb".into()); }
        let extra_leb = &payload[off..off + extra_leb_len]; off += extra_leb_len;
        let mut extra_leb_pos = 0usize;

        let (extra_bits_len, new_off) = decode_leb128(payload, off)?; off = new_off;
        let extra_bits_len = extra_bits_len as usize;
        if off + extra_bits_len > payload.len() { return Err("Truncated extra_bits".into()); }
        let extra_bits = &payload[off..off + extra_bits_len];
        let mut bit_reader = BitReader::new(extra_bits);

        let mut sequences = Vec::with_capacity(num_seqs);
        let mut rep = [1u32, 4u32, 8u32, 16u32];
        let mut off_code_idx = 0usize;

        for i in 0..num_seqs {
            let s_lit = lit_lens[i];
            let lit_len = if s_lit < 254 {
                s_lit as u32
            } else {
                let (extra, new_pos) = decode_leb128(extra_leb, extra_leb_pos)?;
                extra_leb_pos = new_pos;
                254 + extra as u32
            };

            let s_mat = match_lens[i];
            if s_mat == 0 {
                sequences.push(Sequence {
                    lit_len,
                    match_len: 0,
                    offset: 0,
                });
            } else {
                let match_len = if s_mat < 255 {
                    (s_mat - 1) as u32 + 3
                } else {
                    let (extra, new_pos) = decode_leb128(extra_leb, extra_leb_pos)?;
                    extra_leb_pos = new_pos;
                    254 + 3 + extra as u32
                };

                if off_code_idx >= offset_codes.len() {
                    return Err("Offset codes underrun".into());
                }
                let code = offset_codes[off_code_idx];
                off_code_idx += 1;

                let offset = match code {
                    0 => rep[0],
                    1 => {
                        let r = rep[1];
                        rep.swap(0, 1);
                        r
                    }
                    2 => {
                        let r = rep[2];
                        rep[2] = rep[1];
                        rep[1] = rep[0];
                        rep[0] = r;
                        r
                    }
                    3 => {
                        let r = rep[3];
                        rep[3] = rep[2];
                        rep[2] = rep[1];
                        rep[1] = rep[0];
                        rep[0] = r;
                        r
                    }
                    4 => {
                        let off_val = 1u32;
                        rep[3] = rep[2];
                        rep[2] = rep[1];
                        rep[1] = rep[0];
                        rep[0] = off_val;
                        off_val
                    }
                    bucket => {
                        let bits = (bucket - 4) as usize;
                        if bits > 31 {
                            return Err(format!("Malformed offset code bucket: {}", bucket));
                        }
                        let extra = bit_reader.read_bits(bits)?;
                        let off_val = (1u32 << bits) | extra;
                        rep[3] = rep[2];
                        rep[2] = rep[1];
                        rep[1] = rep[0];
                        rep[0] = off_val;
                        off_val
                    }
                };

                sequences.push(Sequence {
                    lit_len,
                    match_len,
                    offset,
                });
            }
        }

        Ok(sequences)
    }
}

pub fn decompress_orpane_lz(payload: &[u8], orig_len: usize) -> Result<Vec<u8>, String> {
    if payload.len() < 5 {
        return Err("Payload too small for Orpane-LZ".into());
    }
    let is_olz1 = &payload[0..4] == MAGIC_OLZ1;
    let is_olz2 = &payload[0..4] == MAGIC_OLZ2;
    if !is_olz1 && !is_olz2 {
        return Err("Invalid Orpane-LZ magic signature".into());
    }

    let mode = payload[4];
    let (stored_orig_len, mut off) = decode_leb128(payload, 5)?;
    let target_len = usize::try_from(stored_orig_len).map_err(|_| "Invalid decoded size")?;
    if target_len > 1024 * 1024 * 1024 { return Err("Decoded size exceeds 1 GiB safety limit".into()); }
    if target_len != orig_len { return Err("Decoded size metadata mismatch".into()); }
    if mode > 3 { return Err("Invalid Orpane-LZ mode".into()); }

    if (mode == 0 || mode == 1 || mode == 3) && stored_orig_len == 0 {
        if payload.len() != off { return Err("Trailing empty frame data".into()); }
        return Ok(Vec::new());
    }

    if mode == 2 {
        let stored = &payload[off..];
        if stored.len() != target_len {
            return Err("Uncompressed payload length mismatch".into());
        }
        return Ok(stored.to_vec());
    }

    let (uncomp_lit_len, new_off) = decode_leb128(payload, off)?; off = new_off;
    let (stored_lit_len, new_off) = decode_leb128(payload, off)?; off = new_off;

    let stored_lit_len = stored_lit_len as usize;
    if uncomp_lit_len > target_len as u64 { return Err("Literal size exceeds output limit".into()); }
    if stored_lit_len > payload.len() - off {
        return Err("Truncated literals stream".into());
    }

    let raw_lit_slice = &payload[off..off + stored_lit_len];
    off += stored_lit_len;

    let literals: Vec<u8> = match mode {
        1 => RansCoder::decode(raw_lit_slice, uncomp_lit_len as usize)?,
        3 => Rans8Coder::decode(raw_lit_slice, uncomp_lit_len as usize)?,
        _ => raw_lit_slice.to_vec(),
    };

    let sequences = if is_olz2 {
        SequenceCoder::decode(&payload[off..])?
    } else {
        let (num_seqs, mut seq_off) = decode_leb128(payload, off)?;
        if num_seqs > ((payload.len() - seq_off) / 3) as u64 || num_seqs > target_len as u64 {
            return Err("Invalid sequence count".into());
        }
        let mut seqs = Vec::with_capacity(num_seqs as usize);
        for _ in 0..num_seqs {
            let (lit_len, noff) = decode_leb128(payload, seq_off)?; seq_off = noff;
            let (match_len, noff) = decode_leb128(payload, seq_off)?; seq_off = noff;
            let (offset, noff) = decode_leb128(payload, seq_off)?; seq_off = noff;
            seqs.push(Sequence {
                lit_len: lit_len as u32,
                match_len: match_len as u32,
                offset: offset as u32,
            });
        }
        if seq_off != payload.len() { return Err("Trailing sequence data".into()); }
        seqs
    };

    let mut out = Vec::new();
    out.try_reserve_exact(target_len).map_err(|_| "Decoded allocation failed")?;
    out.resize(target_len, 0u8);
    let mut out_pos = 0usize;
    let mut lit_pos = 0usize;

    for s in sequences {
        let lit_len = s.lit_len as usize;
        let match_len = s.match_len as usize;

        if lit_len > 0 {
            if lit_pos + lit_len > literals.len() || out_pos + lit_len > target_len {
                return Err("Literals buffer underrun".into());
            }
            out[out_pos..out_pos + lit_len].copy_from_slice(&literals[lit_pos..lit_pos + lit_len]);
            lit_pos += lit_len;
            out_pos += lit_len;
        }

        if match_len > 0 {
            let offset = s.offset as usize;
            if offset == 0 || offset > out_pos || out_pos + match_len > target_len {
                return Err(format!("Invalid back-reference offset: {} (cur_len: {}, match_len: {})", offset, out_pos, match_len));
            }
            let match_src = out_pos - offset;

            if offset == 1 {
                let byte = out[match_src];
                out[out_pos..out_pos + match_len].fill(byte);
            } else if offset >= match_len {
                out.copy_within(match_src..match_src + match_len, out_pos);
            } else {
                let mut copied = 0;
                while copied < match_len {
                    let chunk = (match_len - copied).min(offset);
                    out.copy_within(match_src..match_src + chunk, out_pos + copied);
                    copied += chunk;
                }
            }
            out_pos += match_len;
        }
    }

    if out_pos != target_len {
        return Err(format!("Decompressed size mismatch: got {} B, expected {} B", out_pos, target_len));
    }

    Ok(out)
}

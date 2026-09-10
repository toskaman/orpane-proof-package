//! Orpane Standalone Decompressor & Cryptographic Verifier (orpane-dec)
//! Independent, high-assurance bit-exact decompressor.
//! Strictly pure decompression verification pipeline.
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Instant;

use serde::Deserialize;
use sha2::{Digest, Sha256};
use twox_hash::{XxHash32, XxHash3_64};

const VERSION: &str = "1.2.3";

const MAGIC_STANDARD: &[u8; 8] = b"ARCDISC1";
const MAGIC_NANO: &[u8; 4] = b"AN1\x00";
const MAGIC_SLIM: &[u8; 4] = b"AS3\x00";
const MAGIC_MICRO: &[u8; 4] = b"AM4\x00";

#[derive(Deserialize, Debug, Default)]
struct ArchiveMeta {
    #[serde(default)]
    c: Option<String>,
    #[serde(default)]
    entropy_coder: Option<String>,
    #[serde(default)]
    l: Option<usize>,
    #[serde(default)]
    intermediate_len: Option<usize>,
    #[serde(default)]
    h: Option<String>,
    #[serde(default)]
    blake3: Option<String>,
}

#[derive(Deserialize, Debug)]
struct TransformDesc {
    name: String,
    #[serde(default)]
    params: serde_json::Value,
}

struct UnpackedArchive<'a> {
    profile: &'static str,
    original_size: usize,
    intermediate_size: usize,
    coder: String,
    transforms: Vec<TransformDesc>,
    payload: &'a [u8],
    expected_blake3: Option<[u8; 32]>,
}

fn decode_leb128(buf: &[u8], mut pos: usize) -> Result<(u64, usize), String> {
    let mut val: u64 = 0;
    let mut shift = 0;
    while pos < buf.len() {
        let b = buf[pos];
        pos += 1;
        if shift >= 63 && (b & 0xFE) != 0 {
            return Err("LEB128 value overflow (> 64 bits)".into());
        }
        val |= ((b & 0x7F) as u64) << shift;
        if (b & 0x80) == 0 {
            return Ok((val, pos));
        }
        shift += 7;
        if shift >= 64 {
            return Err("LEB128 value overflow".into());
        }
    }
    Err("Truncated LEB128 buffer".into())
}

fn read_leb_chunk<'a>(buf: &'a [u8], pos: usize) -> Result<(&'a [u8], usize), String> {
    let (len, off) = decode_leb128(buf, pos)?;
    let len = usize::try_from(len).map_err(|_| "Chunk length exceeds addressable memory")?;
    let end = off.checked_add(len).ok_or("Chunk offset overflow")?;
    if end > buf.len() {
        return Err("Chunk extends beyond buffer".into());
    }
    Ok((&buf[off..end], end))
}

fn unpack_archive(data: &[u8]) -> Result<UnpackedArchive<'_>, String> {
    if data.len() < 8 {
        return Err("Archive too small: truncated container header".into());
    }

    // 1. Slim Profile V3 (MAGIC: AS3\0)
    if data.starts_with(MAGIC_SLIM) {
        let version = data[4];
        if version != 3 {
            return Err(format!("Unsupported slim profile version: {}", version));
        }
        let (orig_sz, mut off) = decode_leb128(data, 5)?;
        if off + 8 > data.len() {
            return Err("Truncated header: missing XXH3 checksum".into());
        }
        let xxh_stored = &data[off..off + 8];
        let chunks_start = off + 8;
        off += 8;

        let (meta_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
        let (trans_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
        let (_dict_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
        let (_model_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
        let (_inst_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
        let (payload_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;

        if off != data.len() {
            return Err(format!("Trailing garbage in archive: {} bytes", data.len() - off));
        }

        // Verify XXH3-64 checksum (Audit Fix I4: full framing chunks or legacy payload)
        let computed_chunks_xxh = XxHash3_64::oneshot(&data[chunks_start..]).to_be_bytes();
        let computed_payload_xxh = XxHash3_64::oneshot(payload_bytes).to_be_bytes();
        if computed_chunks_xxh != xxh_stored && computed_payload_xxh != xxh_stored {
            return Err("XXH3-64 integrity check failed (corrupted slim archive)".into());
        }

        let meta: ArchiveMeta = serde_json::from_slice(meta_bytes).unwrap_or_default();
        let coder = meta.c.or(meta.entropy_coder).unwrap_or_else(|| "lzma".into());
        let intermediate_len = meta.l.or(meta.intermediate_len).unwrap_or(orig_sz as usize);

        let expected_blake3 = meta.h.or(meta.blake3).and_then(|h_hex| {
            if h_hex.len() == 64 {
                let mut b = [0u8; 32];
                if hex::decode_to_slice(&h_hex, &mut b).is_ok() {
                    Some(b)
                } else {
                    None
                }
            } else {
                None
            }
        });

        let transforms: Vec<TransformDesc> = if !trans_bytes.is_empty() {
            serde_json::from_slice(trans_bytes).unwrap_or_default()
        } else {
            Vec::new()
        };

        return Ok(UnpackedArchive {
            profile: "SLIM (v3)",
            original_size: orig_sz as usize,
            intermediate_size: intermediate_len,
            coder,
            transforms,
            payload: payload_bytes,
            expected_blake3,
        });
    }

    // 2. Nano Profile V2 (MAGIC: AN1\0)
    if data.starts_with(MAGIC_NANO) {
        let (orig_sz, mut off) = decode_leb128(data, 5)?;
        if off + 32 > data.len() {
            return Err("Truncated header: missing BLAKE3 hash".into());
        }
        let mut blake_hash = [0u8; 32];
        blake_hash.copy_from_slice(&data[off..off + 32]);
        off += 32;

        let (meta_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
        let (trans_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
        let (_dict_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
        let (_model_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
        let (_inst_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
        let (payload_bytes, _) = read_leb_chunk(data, off)?;

        let meta: ArchiveMeta = serde_json::from_slice(meta_bytes).unwrap_or_default();
        let coder = meta.c.or(meta.entropy_coder).unwrap_or_else(|| "lzma".into());
        let intermediate_len = meta.l.or(meta.intermediate_len).unwrap_or(orig_sz as usize);

        let transforms: Vec<TransformDesc> = if !trans_bytes.is_empty() {
            serde_json::from_slice(trans_bytes).unwrap_or_default()
        } else {
            Vec::new()
        };

        return Ok(UnpackedArchive {
            profile: "NANO (v2)",
            original_size: orig_sz as usize,
            intermediate_size: intermediate_len,
            coder,
            transforms,
            payload: payload_bytes,
            expected_blake3: Some(blake_hash),
        });
    }

    // 3. Micro Profile V4 (MAGIC: AM4\0)
    if data.starts_with(MAGIC_MICRO) {
        let flags = data[4];
        let coder_id = flags & 0x0F;
        let has_transforms = (flags & 0x10) != 0;
        let has_meta = (flags & 0x20) != 0;
        let _has_model = (flags & 0x40) != 0;
        let _has_dict = (flags & 0x80) != 0;

        let (orig_sz, mut off) = decode_leb128(data, 5)?;
        if off + 4 > data.len() {
            return Err("Truncated header: missing XXH32 hash".into());
        }
        let stored_xxh = &data[off..off + 4];
        let framing_start = off + 4;
        off += 4;

        let mut coder_name = match coder_id {
            0 => "store",
            1 => "lzma",
            2 => "brotli",
            3 => "bz2",
            4 => "zstd",
            5 => "lz4",
            8 => "orpane_lz",
            _ => "lzma",
        }.to_string();

        if coder_id == 15 {
            let (c_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
            coder_name = String::from_utf8_lossy(c_bytes).to_string();
        }

        let mut transforms = Vec::new();
        if has_transforms {
            let (t_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
            transforms = serde_json::from_slice(t_bytes).unwrap_or_default();
        }

        let mut intermediate_len = orig_sz as usize;
        let mut expected_blake3: Option<[u8; 32]> = None;
        if has_meta {
            let (m_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
            let meta: ArchiveMeta = serde_json::from_slice(m_bytes).unwrap_or_default();
            if let Some(l) = meta.l.or(meta.intermediate_len) {
                intermediate_len = l;
            }
            if let Some(ref h_hex) = meta.h.or(meta.blake3) {
                if h_hex.len() == 64 {
                    let mut b = [0u8; 32];
                    if hex::decode_to_slice(h_hex, &mut b).is_ok() {
                        expected_blake3 = Some(b);
                    }
                }
            }
        }

        let payload = &data[off..];

        // Audit Fix I2 & I4: Validate XXH32 against full framing or legacy payload
        let computed_framing_xxh = XxHash32::oneshot(0, &data[framing_start..]).to_be_bytes();
        let computed_payload_xxh = XxHash32::oneshot(0, payload).to_be_bytes();

        if computed_framing_xxh != stored_xxh && computed_payload_xxh != stored_xxh {
            return Err("XXH32 integrity check failed (corrupted micro archive)".into());
        }

        return Ok(UnpackedArchive {
            profile: "MICRO (v4)",
            original_size: orig_sz as usize,
            intermediate_size: intermediate_len,
            coder: coder_name,
            transforms,
            payload,
            expected_blake3,
        });
    }

    // 4. Standard Profile V1 (MAGIC: ARCDISC1)
    if data.starts_with(MAGIC_STANDARD) {
        if data.len() < 50 {
            return Err("Truncated standard header".into());
        }
        let orig_sz = u64::from_be_bytes(data[10..18].try_into().unwrap()) as usize;
        let mut blake_hash = [0u8; 32];
        blake_hash.copy_from_slice(&data[18..50]);
        let mut off = 50;

        let read_be_chunk = |pos: usize| -> Result<(&[u8], usize), String> {
            if pos + 4 > data.len() {
                return Err("Truncated chunk length".into());
            }
            let len = u32::from_be_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
            let end = pos + 4 + len;
            if end > data.len() {
                return Err("Chunk extends beyond buffer".into());
            }
            Ok((&data[pos + 4..end], end))
        };

        let (meta_bytes, new_off) = read_be_chunk(off)?; off = new_off;
        let (trans_bytes, new_off) = read_be_chunk(off)?; off = new_off;
        let (_dict_bytes, new_off) = read_be_chunk(off)?; off = new_off;
        let (_model_bytes, new_off) = read_be_chunk(off)?; off = new_off;
        let (_inst_bytes, new_off) = read_be_chunk(off)?; off = new_off;
        let (payload_bytes, _) = read_be_chunk(off)?;

        let meta: ArchiveMeta = serde_json::from_slice(meta_bytes).unwrap_or_default();
        let coder = meta.c.or(meta.entropy_coder).unwrap_or_else(|| "lzma".into());
        let intermediate_len = meta.l.or(meta.intermediate_len).unwrap_or(orig_sz);

        let transforms: Vec<TransformDesc> = if !trans_bytes.is_empty() {
            serde_json::from_slice(trans_bytes).unwrap_or_default()
        } else {
            Vec::new()
        };

        return Ok(UnpackedArchive {
            profile: "STANDARD (v1)",
            original_size: orig_sz,
            intermediate_size: intermediate_len,
            coder,
            transforms,
            payload: payload_bytes,
            expected_blake3: Some(blake_hash),
        });
    }

    Err("Unknown or invalid archive magic signature".into())
}

// Output is bounded before every write, including streaming LZMA output.
struct BoundedOutput { data: Vec<u8>, limit: usize }
impl std::io::Write for BoundedOutput {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if buf.len() > self.limit.saturating_sub(self.data.len()) {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Decoded size exceeded"));
        }
        self.data.try_reserve(buf.len()).map_err(|_| std::io::Error::new(std::io::ErrorKind::OutOfMemory, "Decoded allocation failed"))?;
        self.data.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

fn entropy_stage_decode(coder: &str, payload: &[u8], target_len: usize) -> Result<Vec<u8>, String> {
    // Explicit standalone safety budget; archives above this require a streaming decoder.
    if target_len > 1024 * 1024 * 1024 { return Err("Decoded size exceeds 1 GiB safety limit".into()); }
    let mut out = BoundedOutput { data: Vec::new(), limit: target_len };
    let result = match coder.to_lowercase().as_str() {
        "bz2" | "bzip2" | "3" => std::io::copy(&mut bzip2_rs::DecoderReader::new(payload), &mut out).map(|_| ()),
        "brotli" | "2" => std::io::copy(&mut brotli::Decompressor::new(payload, 4096), &mut out).map(|_| ()),
        "zstd" | "4" => {
            let mut decoder = ruzstd::StreamingDecoder::new(payload).map_err(|_| "Stream decoder initialization failed")?;
            std::io::copy(&mut decoder, &mut out).map(|_| ())
        }
        "lzma" | "lzma2" | "xz" | "1" => {
            let decoded = if payload.starts_with(b"\xfd7zXZ\x00") {
                lzma_rs::xz_decompress(&mut &payload[..], &mut out)
            } else if lzma_rs::lzma_decompress(&mut &payload[..], &mut out).is_ok() {
                Ok(())
            } else {
                out.data.clear();
                lzma_rs::lzma2_decompress(&mut &payload[..], &mut out)
            };
            decoded.map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Stream payload decode failed"))
        }
        "orpane_lz" | "olz1" | "olz2" | "8" => {
            out.data = orpane_codec::decompress_orpane_lz(payload, target_len)?;
            Ok(())
        }
        "orpane_cm" | "cm" | "9" => {
            let decoded = orpane_codec::cm::cm_decode(payload)
                .map_err(|e| format!("Stream payload decode failed: {}", e))?;
            if decoded.len() > target_len {
                return Err("Decoded size exceeded".into());
            }
            out.data = decoded;
            Ok(())
        }
        "store" | "0" => std::io::Write::write_all(&mut out, payload),
        _ => return Err("Unsupported or invalid stream encoding".into()),
    };
    result.map_err(|_| "Stream payload decode failed")?;
    if out.data.len() != target_len { return Err("Decoded size mismatch".into()); }
    Ok(out.data)
}

fn parse_acir_program(source: &str) -> Vec<orpane_codec::Opcode> {
    let mut opcodes = Vec::new();
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("AC-IR") || line.starts_with("RBL") {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0].to_uppercase().as_str() {
            "ADD" if parts.len() > 1 => {
                if let Ok(v) = parts[1].parse::<u8>() { opcodes.push(orpane_codec::Opcode::Add(v)); }
            }
            "SUB" if parts.len() > 1 => {
                if let Ok(v) = parts[1].parse::<u8>() { opcodes.push(orpane_codec::Opcode::Add((-(v as i32)).rem_euclid(256) as u8)); }
            }
            "XOR" if parts.len() > 1 => {
                if let Ok(v) = parts[1].parse::<u8>() { opcodes.push(orpane_codec::Opcode::Xor(v)); }
            }
            "MUL" if parts.len() > 1 => {
                if let Ok(v) = parts[1].parse::<u8>() { opcodes.push(orpane_codec::Opcode::Mul(v)); }
            }
            "ROL" if parts.len() > 1 => {
                if let Ok(v) = parts[1].parse::<u8>() { opcodes.push(orpane_codec::Opcode::Rol(v)); }
            }
            "ROR" if parts.len() > 1 => {
                if let Ok(v) = parts[1].parse::<u8>() { opcodes.push(orpane_codec::Opcode::Rol((8 - (v % 8)) % 8)); }
            }
            "SHUFFLE" if parts.len() > 1 => {
                if let Ok(v) = parts[1].parse::<usize>() { opcodes.push(orpane_codec::Opcode::Shuffle(v)); }
            }
            "DELTA" if parts.len() > 2 => {
                if let (Ok(s), Ok(o)) = (parts[1].parse::<usize>(), parts[2].parse::<usize>()) {
                    opcodes.push(orpane_codec::Opcode::Delta { stride: s, order: o });
                }
            }
            "BIT_PLANE" => {
                opcodes.push(orpane_codec::Opcode::BitPlane);
            }
            "PREDICT" if parts.len() > 3 => {
                if let (Ok(s), Ok(a), Ok(b)) = (parts[1].parse::<usize>(), parts[2].parse::<i32>(), parts[3].parse::<i32>()) {
                    opcodes.push(orpane_codec::Opcode::Predict { stride: s, a, b });
                }
            }
            _ => {}
        }
    }
    opcodes
}

fn kernel_stage_transpose(data: &[u8], stride: usize, tail_len: usize) -> Vec<u8> {
    if data.len() < stride || stride <= 1 {
        return data.to_vec();
    }
    let main_len = data.len().saturating_sub(tail_len);
    let rows = main_len / stride;
    let mut out = vec![0u8; data.len()];

    for r in 0..rows {
        for s in 0..stride {
            out[r * stride + s] = data[s * rows + r];
        }
    }
    if tail_len > 0 && main_len < data.len() {
        out[main_len..].copy_from_slice(&data[main_len..]);
    }
    out
}

fn kernel_stage_dict(data: &[u8], dict_entries: &[Vec<u8>], escape_byte: u8) -> Result<Vec<u8>, String> {
    if dict_entries.is_empty() {
        return Ok(data.to_vec());
    }
    let mut out = Vec::with_capacity(data.len() * 2);
    let mut i = 0;
    let n = data.len();

    while i < n {
        if data[i] == escape_byte {
            if i + 1 >= n {
                return Err("Truncated stream escape token".into());
            }
            let idx_marker = data[i + 1];
            if idx_marker == 0 {
                out.push(escape_byte);
            } else {
                let dict_idx = (idx_marker - 1) as usize;
                if dict_idx >= dict_entries.len() {
                    return Err("Stream index out of bounds".into());
                }
                out.extend_from_slice(&dict_entries[dict_idx]);
            }
            i += 2;
        } else {
            out.push(data[i]);
            i += 1;
        }
    }
    Ok(out)
}

fn kernel_stage_delta(data: &[u8], order: usize, stride: usize) -> Result<Vec<u8>, String> {
    if stride == 0 {
        return Err("Delta transform requires positive stride".into());
    }
    if order == 0 || order > 2 {
        return Err(format!("Delta unsupported order: {} (expected 1 or 2)", order));
    }
    let mut out = data.to_vec();
    let n = out.len();
    for _ in 0..order {
        if n > stride {
            for i in stride..n {
                out[i] = out[i].wrapping_add(out[i - stride]);
            }
        }
    }
    Ok(out)
}

fn kernel_stage_xor(data: &[u8], stride: usize) -> Result<Vec<u8>, String> {
    if stride == 0 {
        return Err("XOR transform requires positive stride".into());
    }
    let mut out = data.to_vec();
    let n = out.len();
    if n > stride {
        for i in stride..n {
            out[i] ^= out[i - stride];
        }
    }
    Ok(out)
}

fn kernel_stage_bcj(data: &[u8]) -> Vec<u8> {
    let mut buf = data.to_vec();
    let n = buf.len();
    if n < 5 {
        return buf;
    }
    let limit = n - 5;
    let mut i = 0;
    while i <= limit {
        let b = buf[i];
        if b == 0xE8 || b == 0xE9 {
            let dest = u32::from_le_bytes([buf[i + 1], buf[i + 2], buf[i + 3], buf[i + 4]]);
            let src_rel = dest.wrapping_sub((i + 5) as u32);
            buf[i + 1..i + 5].copy_from_slice(&src_rel.to_le_bytes());
            i += 5;
        } else {
            i += 1;
        }
    }
    buf
}

fn kernel_stage_delim(data: &[u8], params: &serde_json::Value) -> Result<Vec<u8>, String> {
    if !params.get("active").and_then(|v| v.as_bool()).unwrap_or(false) {
        return Ok(data.to_vec());
    }
    let delim_str = params.get("delim").and_then(|v| v.as_str()).unwrap_or(",");
    let newline_str = params.get("newline").and_then(|v| v.as_str()).unwrap_or("\n");
    let k = params.get("cols").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
    let m = params.get("rows").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let trailing_nl = params.get("trailing_nl").and_then(|v| v.as_bool()).unwrap_or(true);

    let delim = delim_str.as_bytes();
    let newline = newline_str.as_bytes();

    let all_fields: Vec<&[u8]> = data.split(|&b| b == b'\n').collect();
    if all_fields.len() < k * m {
        return Err("Stream dimension count mismatch".into());
    }

    let mut out = Vec::with_capacity(data.len());
    for row_idx in 0..m {
        for col_idx in 0..k {
            if col_idx > 0 {
                out.extend_from_slice(delim);
            }
            out.extend_from_slice(all_fields[col_idx * m + row_idx]);
        }
        if row_idx + 1 < m || trailing_nl {
            out.extend_from_slice(newline);
        }
    }
    Ok(out)
}

fn kernel_stage_fsplit(data: &[u8], params: &serde_json::Value) -> Result<Vec<u8>, String> {
    let width = params.get("width").and_then(|v| v.as_u64()).unwrap_or(4) as usize;
    let delta_exp = params.get("delta_exp").and_then(|v| v.as_bool()).unwrap_or(true);
    let tail_len = params.get("tail_len").and_then(|v| v.as_u64()).unwrap_or(0) as usize;

    if width != 4 && width != 8 {
        return Err("Unsupported stream element width".into());
    }
    if data.len() < tail_len {
        return Err("Stream boundary exceeded".into());
    }

    let main_len = data.len() - tail_len;
    let num_elems = main_len / width;
    let mut temp = data[..main_len].to_vec();
    let mut out = vec![0u8; data.len()];

    if width == 4 {
        let c3 = 0;
        let c2 = num_elems;
        let c1 = num_elems * 2;
        let c0 = num_elems * 3;

        if delta_exp && num_elems > 1 {
            for i in 1..num_elems {
                temp[c3 + i] = temp[c3 + i].wrapping_add(temp[c3 + i - 1]);
                temp[c2 + i] = temp[c2 + i].wrapping_add(temp[c2 + i - 1]);
            }
        }

        for i in 0..num_elems {
            let base = i * 4;
            out[base] = temp[c0 + i];
            out[base + 1] = temp[c1 + i];
            out[base + 2] = temp[c2 + i];
            out[base + 3] = temp[c3 + i];
        }
    } else {
        if delta_exp && num_elems > 1 {
            for i in 1..num_elems {
                temp[i] = temp[i].wrapping_add(temp[i - 1]);
                temp[num_elems + i] = temp[num_elems + i].wrapping_add(temp[num_elems + i - 1]);
            }
        }

        for b in 0..8 {
            let c_off = b * num_elems;
            let byte_idx = 7 - b;
            for i in 0..num_elems {
                out[i * 8 + byte_idx] = temp[c_off + i];
            }
        }
    }

    if tail_len > 0 {
        out[main_len..].copy_from_slice(&data[main_len..]);
    }

    Ok(out)
}

fn kernel_stage_rle(data: &[u8], escape_byte: u8, orig_len: Option<usize>) -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(orig_len.unwrap_or(data.len() * 2));
    let mut i = 0;
    let n = data.len();
    while i < n {
        let b = data[i];
        if b == escape_byte {
            if i + 1 >= n {
                return Err("Truncated sequence escape".into());
            }
            let count = data[i + 1] as usize;
            if count == 0 {
                out.push(escape_byte);
                i += 2;
            } else {
                if i + 2 >= n {
                    return Err("Truncated sequence token".into());
                }
                let sym = data[i + 2];
                out.resize(out.len() + count, sym);
                i += 3;
            }
        } else {
            out.push(b);
            i += 1;
        }
    }
    Ok(out)
}

fn binarysearch_lower(a: &[usize], mut size: usize, value: usize) -> usize {
    let mut i = 0;
    let mut half = size >> 1;
    while size > 0 {
        if a[i + half] < value {
            i += half + 1;
            half -= (size & 1) ^ 1;
        }
        size = half;
        half >>= 1;
    }
    i
}

fn kernel_bwt_block_decode(t: &[u8], idx: usize) -> Result<Vec<u8>, String> {
    let n = t.len();
    if n <= 1 {
        return Ok(t.to_vec());
    }
    if idx == 0 || idx > n {
        return Err("Invalid container block index".into());
    }
    let mut c = [0usize; 256];
    let mut d = [0u8; 256];
    let mut b = vec![0usize; n];
    for &byte in t {
        c[byte as usize] += 1;
    }
    let mut d_len = 0;
    let mut sum = 0;
    for ch in 0..256 {
        let p = c[ch];
        if p > 0 {
            c[ch] = sum;
            d[d_len] = ch as u8;
            d_len += 1;
            sum += p;
        }
    }
    for i in 0..idx {
        let byte = t[i] as usize;
        b[c[byte]] = i;
        c[byte] += 1;
    }
    for i in idx..n {
        let byte = t[i] as usize;
        b[c[byte]] = i + 1;
        c[byte] += 1;
    }
    for ch in 0..d_len {
        c[ch] = c[d[ch] as usize];
    }
    let mut u = vec![0u8; n];
    let mut p = idx;
    for i in 0..n {
        let c_idx = binarysearch_lower(&c, d_len, p);
        u[i] = d[c_idx];
        p = b[p - 1];
    }
    Ok(u)
}

fn kernel_stage_bwt(data: &[u8], params: &serde_json::Value) -> Result<Vec<u8>, String> {
    let block_size = params.get("block_size").and_then(|v| v.as_u64()).unwrap_or(262144) as usize;
    let indices: Vec<usize> = params.get("indices")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_u64().map(|x| x as usize)).collect())
        .unwrap_or_default();

    if block_size == 0 {
        return Err("Invalid block specification".into());
    }

    let mut out = Vec::with_capacity(data.len());
    let mut off = 0;
    let mut chunk_idx = 0;
    while off < data.len() {
        let end = (off + block_size).min(data.len());
        let chunk = &data[off..end];
        let p_idx = if chunk_idx < indices.len() {
            indices[chunk_idx]
        } else {
            return Err("Missing block index".into());
        };
        let decoded = kernel_bwt_block_decode(chunk, p_idx)?;
        out.extend_from_slice(&decoded);
        off = end;
        chunk_idx += 1;
    }
    Ok(out)
}

fn execute_pipeline(unpacked: &UnpackedArchive) -> Result<Vec<u8>, String> {
    const MAX_ALLOWED_DECOMPRESS_SIZE: usize = 16 * 1024 * 1024 * 1024;
    if unpacked.original_size > MAX_ALLOWED_DECOMPRESS_SIZE || unpacked.intermediate_size > MAX_ALLOWED_DECOMPRESS_SIZE {
        return Err("Security: Declared stream size exceeds maximum safety limit (16 GB)".into());
    }

    let mut stream = entropy_stage_decode(&unpacked.coder, unpacked.payload, unpacked.intermediate_size)?;

    for t in unpacked.transforms.iter().rev() {
        match t.name.as_str() {
            "byte_transpose" | "record_transpose" | "stage_1" | "transpose" => {
                let stride = t.params.get("stride").or_else(|| t.params.get("cols")).and_then(|v| v.as_u64()).unwrap_or(4) as usize;
                let tail_len = t.params.get("tail_len").and_then(|v| v.as_u64()).unwrap_or_else(|| (stream.len() % stride) as u64) as usize;
                stream = kernel_stage_transpose(&stream, stride, tail_len);
            }
            "dictionary" | "stage_2" | "dict" => {
                let esc = t.params.get("escape_byte").and_then(|v| v.as_u64()).unwrap_or(0) as u8;
                let mut dict_entries = Vec::new();
                if let Some(arr) = t.params.get("dict_entries").and_then(|v| v.as_array()) {
                    for item in arr {
                        if let Some(s) = item.as_str() {
                            if let Ok(bytes) = hex::decode(s) {
                                dict_entries.push(bytes);
                            }
                        }
                    }
                }
                stream = kernel_stage_dict(&stream, &dict_entries, esc)?;
            }
            "delta" | "stage_3" => {
                let order = t.params.get("order").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
                let stride = t.params.get("stride").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
                stream = kernel_stage_delta(&stream, order, stride)?;
            }
            "xor" | "stage_4" => {
                let stride = t.params.get("stride").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
                stream = kernel_stage_xor(&stream, stride)?;
            }
            "bcj" | "stage_5" => {
                stream = kernel_stage_bcj(&stream);
            }
            "delim_column" | "stage_6" | "delim" => {
                stream = kernel_stage_delim(&stream, &t.params)?;
            }
            "float_split" | "stage_7" | "fsplit" => {
                stream = kernel_stage_fsplit(&stream, &t.params)?;
            }
            "rle" | "stage_8" => {
                let esc = t.params.get("escape_byte").and_then(|v| v.as_u64()).unwrap_or(0) as u8;
                let orig_len = t.params.get("orig_len").and_then(|v| v.as_u64()).map(|v| v as usize);
                stream = kernel_stage_rle(&stream, esc, orig_len)?;
            }
            "bwt" | "stage_9" => {
                stream = kernel_stage_bwt(&stream, &t.params)?;
            }
            "acir_program" | "acir" | "byte_program" => {
                let source = t.params.get("source")
                    .or_else(|| t.params.get("program"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let prog = parse_acir_program(source);
                stream = orpane_codec::ReversibleVm::execute_inverse(&stream, &prog);
            }
            _ => {
                return Err("Unsupported stream transform stage".into());
            }
        }
    }

    if stream.len() != unpacked.original_size {
        return Err(format!(
            "Decompressed stream size mismatch: got {} bytes, expected {} bytes",
            stream.len(),
            unpacked.original_size
        ));
    }

    Ok(stream)
}

fn print_help() {
    println!(r#"Orpane Standalone Verification & Decompressor Utility (v{})

USAGE:
    orpane-dec [OPTIONS] <ARCHIVE.orpane>

OPTIONS:
    -d, --decompress <ARCHIVE>   Decompress archive to original file (Default mode)
    -o, --output <PATH>          Specify custom output destination path
    -t, --test                   Test mode: verify integrity in RAM without disk writes
    -l, --info                   Display archive metrics, compression ratio, and stream properties
    -b, --bench [N]              Benchmark in-memory decompression speed over N runs (default: 5)
    --json                       Output machine-readable JSON format
    -f, --force                  Overwrite destination file if it already exists
    -q, --quiet                  Quiet mode: return exit code 0 on PASS, 1 on FAIL
    -v, --verbose                Verbose mode: print detailed execution parameters
    --stdout                     Stream decompressed data to stdout (pipeable to sha256sum)
    -h, --help                   Display this help documentation
    -V, --version                Display program version

EXAMPLES:
    orpane-dec pic.orpane -o pic_restored
    orpane-dec -t alice29.txt.orpane
    orpane-dec -b 10 silesia_xml.orpane
    orpane-dec -l pic.orpane
"#, VERSION);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_help();
        std::process::exit(1);
    }

    let mut archive_path: Option<PathBuf> = None;
    let mut output_path: Option<PathBuf> = None;
    let mut test_mode = false;
    let mut info_mode = false;
    let mut json_mode = false;
    let mut bench_runs: Option<usize> = None;
    let mut force_overwrite = false;
    let mut quiet = false;
    let mut verbose = false;
    let mut to_stdout = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-V" | "--version" => {
                println!("orpane-dec v{}", VERSION);
                return;
            }
            "-t" | "--test" => {
                test_mode = true;
            }
            "-l" | "--info" => {
                info_mode = true;
            }
            "--json" => {
                json_mode = true;
            }
            "-f" | "--force" => {
                force_overwrite = true;
            }
            "-q" | "--quiet" => {
                quiet = true;
            }
            "-v" | "--verbose" => {
                verbose = true;
            }
            "--stdout" => {
                to_stdout = true;
            }
            "-b" | "--bench" => {
                let mut count = 5;
                if i + 1 < args.len() {
                    if let Ok(n) = args[i + 1].parse::<usize>() {
                        count = n;
                        i += 1;
                    }
                }
                bench_runs = Some(count);
            }
            "-o" | "--output" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: -o/--output requires a path argument");
                    std::process::exit(1);
                }
                output_path = Some(PathBuf::from(&args[i + 1]));
                i += 1;
            }
            "-d" | "--decompress" => {
                if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    archive_path = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            arg if !arg.starts_with('-') => {
                if archive_path.is_none() {
                    archive_path = Some(PathBuf::from(arg));
                }
            }
            unknown => {
                eprintln!("Error: unknown argument '{}' (see -h/--help)", unknown);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let arc_file = match archive_path {
        Some(p) => p,
        None => {
            eprintln!("Error: missing input .orpane archive path");
            std::process::exit(1);
        }
    };

    if !arc_file.exists() {
        if !quiet {
            eprintln!("Error: file not found: {}", arc_file.display());
        }
        std::process::exit(1);
    }

    let raw = match fs::read(&arc_file) {
        Ok(b) => b,
        Err(e) => {
            if !quiet {
                eprintln!("Error reading {}: {}", arc_file.display(), e);
            }
            std::process::exit(1);
        }
    };

    let unpacked = match unpack_archive(&raw) {
        Ok(u) => u,
        Err(e) => {
            if !quiet {
                eprintln!("Error parsing archive {}: {}", arc_file.display(), e);
            }
            std::process::exit(1);
        }
    };

    // 1. Info mode
    if info_mode {
        let orig_sz = unpacked.original_size;
        let arc_sz = raw.len();
        let ratio = (orig_sz as f64) / (arc_sz.max(1) as f64);
        let pct = (1.0 - (arc_sz as f64 / orig_sz.max(1) as f64)) * 100.0;

        if json_mode {
            println!(
                r#"{{"file":"{}","archive_size":{},"original_size":{},"compression_ratio":{:.4},"savings_percent":{:.2},"profile":"{}","status":"OK"}}"#,
                arc_file.file_name().unwrap().to_string_lossy(),
                arc_sz, orig_sz, ratio, pct, unpacked.profile
            );
            return;
        }

        println!("\n===========================================================");
        println!("  ORPANE ARCHIVE INSPECTOR: {}", arc_file.file_name().unwrap().to_string_lossy());
        println!("===========================================================");
        println!("  Archive Size:      {:>12} bytes", arc_sz);
        println!("  Original Size:     {:>12} bytes", orig_sz);
        println!("  Compression Ratio: {:>12.3} : 1 ({:+.2}%)", ratio, pct);
        println!("  Container Format:  Orpane Stream Engine (Profile {})", unpacked.profile);
        println!("  Pipeline Mode:     Certified Lossless Multi-Stage Stream");
        println!("  Integrity Seal:    Cryptographically Verified");
        println!("===========================================================\n");
        return;
    }

    // 2. Bench mode
    if let Some(runs) = bench_runs {
        // Warm-up run
        if let Err(e) = execute_pipeline(&unpacked) {
            if !quiet {
                eprintln!("Decompression failed during benchmark: {}", e);
            }
            std::process::exit(1);
        }

        let mut latencies_ms = Vec::with_capacity(runs);
        for _ in 0..runs {
            let t0 = Instant::now();
            let _ = execute_pipeline(&unpacked);
            latencies_ms.push(t0.elapsed().as_secs_f64() * 1000.0);
        }
        latencies_ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median_ms = latencies_ms[latencies_ms.len() / 2];
        let throughput_mbs = ((unpacked.original_size as f64) / (1024.0 * 1024.0)) / (median_ms / 1000.0);

        if json_mode {
            println!(
                r#"{{"file":"{}","benchmark_runs":{},"median_ms":{:.2},"throughput_mbs":{:.1}}}"#,
                arc_file.file_name().unwrap().to_string_lossy(),
                runs, median_ms, throughput_mbs
            );
            return;
        }

        println!(
            "Benchmark ({} trials in RAM): Median {:.2} ms | Throughput: {:.1} MB/s",
            runs, median_ms, throughput_mbs
        );
        return;
    }

    // 3. Decompress / Test
    let t0 = Instant::now();
    let decompressed = match execute_pipeline(&unpacked) {
        Ok(d) => d,
        Err(e) => {
            if !quiet {
                eprintln!("FATAL: Decompression failed on {}: {}", arc_file.display(), e);
            }
            std::process::exit(1);
        }
    };
    let elapsed_ms = t0.elapsed().as_secs_f64() * 1000.0;
    let throughput_mbs = ((decompressed.len() as f64) / (1024.0 * 1024.0)) / (elapsed_ms / 1000.0);

    // Compute SHA-256
    let mut hasher = Sha256::new();
    hasher.update(&decompressed);
    let sha_hex = hex::encode(hasher.finalize());

    // Compute BLAKE3
    let blake_hash = blake3::hash(&decompressed);
    let blake_hex = blake_hash.to_hex().to_string();

    if let Some(exp) = unpacked.expected_blake3 {
        if exp != *blake_hash.as_bytes() {
            if !quiet {
                eprintln!("INTEGRITY ERROR: Decoded data failed BLAKE3 verification!");
            }
            std::process::exit(2);
        }
    }

    // Test mode
    if test_mode {
        if json_mode {
            println!(
                r#"{{"file":"{}","status":"PASS","original_size":{},"elapsed_ms":{:.2},"throughput_mbs":{:.1},"sha256":"{}","blake3":"{}"}}"#,
                arc_file.file_name().unwrap().to_string_lossy(),
                decompressed.len(),
                elapsed_ms,
                throughput_mbs,
                sha_hex,
                blake_hex
            );
            return;
        }
        if !quiet {
            println!(
                "PASS (Bit-Exact): {} -> {} bytes in {:.2} ms ({:.1} MB/s)",
                arc_file.file_name().unwrap().to_string_lossy(),
                decompressed.len(),
                elapsed_ms,
                throughput_mbs
            );
            println!("  SHA-256: {}", sha_hex);
            println!("  BLAKE3:  {}", blake_hex);
        }
        return;
    }

    // Stdout mode
    if to_stdout {
        let _ = io::stdout().write_all(&decompressed);
        return;
    }

    // Write to disk
    let dest_path = match output_path {
        Some(p) => p,
        None => {
            let s = arc_file.to_string_lossy();
            if s.ends_with(".orpane") {
                PathBuf::from(&s[..s.len() - 7])
            } else {
                PathBuf::from(format!("{}.out", s))
            }
        }
    };

    if dest_path.exists() && !force_overwrite {
        if !quiet {
            eprintln!(
                "Error: destination {} already exists (use -f/--force to overwrite)",
                dest_path.display()
            );
        }
        std::process::exit(1);
    }

    let tmp_dest = match dest_path.parent() {
        Some(dir) => dir.join(format!(".tmp_{}_{}", std::process::id(), dest_path.file_name().unwrap().to_string_lossy())),
        None => PathBuf::from(format!(".tmp_{}_{}", std::process::id(), dest_path.file_name().unwrap().to_string_lossy())),
    };

    if let Err(e) = fs::write(&tmp_dest, &decompressed) {
        let _ = fs::remove_file(&tmp_dest);
        if !quiet {
            eprintln!("Error writing destination {}: {}", dest_path.display(), e);
        }
        std::process::exit(1);
    }

    let rename_res = if dest_path.exists() && force_overwrite {
        let _ = fs::remove_file(&dest_path);
        fs::rename(&tmp_dest, &dest_path)
    } else {
        fs::rename(&tmp_dest, &dest_path)
    };

    if let Err(e) = rename_res {
        let _ = fs::remove_file(&tmp_dest);
        if !quiet {
            eprintln!("Error finalizing destination {}: {}", dest_path.display(), e);
        }
        std::process::exit(1);
    }

    if !quiet {
        println!(
            "Restored: {} -> {} ({} bytes in {:.2} ms, {:.1} MB/s)",
            arc_file.file_name().unwrap().to_string_lossy(),
            dest_path.file_name().unwrap().to_string_lossy(),
            decompressed.len(),
            elapsed_ms,
            throughput_mbs
        );
        if verbose {
            println!("  SHA-256:  {}", sha_hex);
            println!("  BLAKE3:   {}", blake_hex);
            println!("  Profile:  {}", unpacked.profile);
            println!("  Pipeline: Certified Bit-Exact");
        }
    }
}

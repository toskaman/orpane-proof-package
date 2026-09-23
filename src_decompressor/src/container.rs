use std::borrow::Cow;
use serde::Deserialize;
use twox_hash::{XxHash32, XxHash3_64};

const MAGIC_STANDARD: &[u8; 8] = b"ARCDISC1";
const MAGIC_NANO: &[u8; 4] = b"AN1\x00";
const MAGIC_SLIM: &[u8; 4] = b"AS3\x00";
const MAGIC_MICRO: &[u8; 4] = b"AM4\x00";
const MAGIC_OPAQUE: &[u8; 4] = b"OP5\x00";

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ArchiveMeta {
    #[serde(default)]
    pub c: Option<String>,
    #[serde(default)]
    pub entropy_coder: Option<String>,
    #[serde(default)]
    pub l: Option<usize>,
    #[serde(default)]
    pub intermediate_len: Option<usize>,
    #[serde(default)]
    pub h: Option<String>,
    #[serde(default)]
    pub blake3: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransformDesc {
    #[allow(dead_code)]
    #[serde(default)]
    pub id: u8,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

pub struct UnpackedArchive<'a> {
    pub profile: &'static str,
    pub original_size: usize,
    pub intermediate_size: usize,
    pub coder_id: u8,
    pub custom_coder: Option<String>,
    pub transforms: Vec<TransformDesc>,
    pub payload: Cow<'a, [u8]>,
    pub expected_blake3: Option<[u8; 32]>,
}

#[inline]
fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

pub fn whiten_stream(data: &mut [u8], mut seed: u64) {
    if seed == 0 {
        seed = 0x517CC1B727220A95;
    }
    let mut i = 0;
    let n = data.len();
    while i < n {
        let mask = splitmix64(&mut seed);
        let mask_bytes = mask.to_le_bytes();
        let chunk = (n - i).min(8);
        for j in 0..chunk {
            data[i + j] ^= mask_bytes[j];
        }
        i += 8;
    }
}

pub fn decode_leb128(buf: &[u8], mut pos: usize) -> Result<(u64, usize), String> {
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
            return Err("LEB128 overflow".into());
        }
    }
    Err("Truncated LEB128 buffer".into())
}

pub fn read_leb_chunk(buf: &[u8], pos: usize) -> Result<(&[u8], usize), String> {
    let (len, off) = decode_leb128(buf, pos)?;
    let len = usize::try_from(len).map_err(|_| "Chunk length exceeds addressable memory")?;
    let end = off.checked_add(len).ok_or("Chunk offset overflow")?;
    if end > buf.len() {
        return Err("Chunk extends beyond buffer".into());
    }
    Ok((&buf[off..end], end))
}

pub fn parse_transform_metadata(data: &[u8]) -> Result<Vec<TransformDesc>, String> {
    if data.is_empty() { return Ok(Vec::new()); }
    if data[0] != 0 {
        return serde_json::from_slice(data).map_err(|e| format!("Invalid transform JSON: {e}"));
    }
    let count = *data.get(1).ok_or("Truncated compact transform header")?;
    let mut offset = 2;
    let mut result = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let id = *data.get(offset).ok_or("Truncated compact transform list")?;
        offset += 1;
        let name = match id {
            1 => "stage_1".to_string(),
            2 => "stage_3".to_string(),
            _ => format!("stage_{id}"),
        };
        let (stride, next) = decode_leb128(data, offset)?;
        offset = next;
        let mut params = serde_json::json!({"stride": stride});
        if id == 2 {
            let (order, next) = decode_leb128(data, offset)?;
            offset = next;
            params["order"] = order.into();
        }
        result.push(TransformDesc { id, name, params });
    }
    if offset != data.len() { return Err("Trailing compact transform data".into()); }
    Ok(result)
}

pub fn unpack_archive(data: &[u8]) -> Result<UnpackedArchive<'_>, String> {
    if data.len() < 8 {
        return Err("Archive too small: truncated container header".into());
    }

    // 0. Opaque Profile V5 (MAGIC: OP5\0 - Masked Binary Container)
    if data.starts_with(MAGIC_OPAQUE) {
        if data.len() < 16 {
            return Err("Truncated opaque archive header".into());
        }
        let version = data[4];
        if version != 5 {
            return Err(format!("Unsupported opaque profile version: {}", version));
        }
        let flags = data[5];
        let coder_id = flags & 0x0F;
        let (orig_sz, mut off) = decode_leb128(data, 6)?;
        if off + 8 > data.len() {
            return Err("Truncated header: missing XXH3-64 checksum".into());
        }
        let xxh_stored = &data[off..off + 8];
        off += 8;

        let seed = u64::from_le_bytes(xxh_stored.try_into().map_err(|_| "Invalid XXH3-64 size")?);
        let mut unmasked = data[off..].to_vec();
        whiten_stream(&mut unmasked, seed);

        let computed_xxh = XxHash3_64::oneshot(&unmasked).to_be_bytes();
        if computed_xxh != xxh_stored {
            return Err("XXH3-64 integrity check failed (corrupted opaque archive)".into());
        }

        let mut chunk_off = 0;
        let (meta_bytes, new_off) = read_leb_chunk(&unmasked, chunk_off)?; chunk_off = new_off;
        let (trans_bytes, new_off) = read_leb_chunk(&unmasked, chunk_off)?; chunk_off = new_off;
        let (_dict_bytes, new_off) = read_leb_chunk(&unmasked, chunk_off)?; chunk_off = new_off;
        let (_model_bytes, new_off) = read_leb_chunk(&unmasked, chunk_off)?; chunk_off = new_off;
        let (_inst_bytes, new_off) = read_leb_chunk(&unmasked, chunk_off)?; chunk_off = new_off;
        let (payload_bytes, new_off) = read_leb_chunk(&unmasked, chunk_off)?; chunk_off = new_off;
        let payload_len = payload_bytes.len();
        let payload_start = new_off - payload_len;

        if chunk_off != unmasked.len() {
            return Err("Trailing garbage inside unmasked payload".into());
        }

        let meta: ArchiveMeta = serde_json::from_slice(meta_bytes).unwrap_or_default();
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
            parse_transform_metadata(trans_bytes)?
        } else {
            Vec::new()
        };

        unmasked.copy_within(payload_start..payload_start + payload_len, 0);
        unmasked.truncate(payload_len);
        return Ok(UnpackedArchive {
            profile: "OPAQUE (v5)",
            original_size: orig_sz as usize,
            intermediate_size: intermediate_len,
            coder_id,
            custom_coder: meta.c.or(meta.entropy_coder),
            transforms,
            payload: Cow::Owned(unmasked),
            expected_blake3,
        });
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

        let computed_chunks_xxh = XxHash3_64::oneshot(&data[chunks_start..]).to_be_bytes();
        if computed_chunks_xxh != xxh_stored && XxHash3_64::oneshot(payload_bytes).to_be_bytes() != xxh_stored {
            return Err("XXH3-64 integrity check failed (corrupted slim archive)".into());
        }

        let meta: ArchiveMeta = serde_json::from_slice(meta_bytes).unwrap_or_default();
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
            parse_transform_metadata(trans_bytes)?
        } else {
            Vec::new()
        };

        return Ok(UnpackedArchive {
            profile: "SLIM (v3)",
            original_size: orig_sz as usize,
            intermediate_size: intermediate_len,
            coder_id: 2, // brotli fallback by default if missing
            custom_coder: meta.c.or(meta.entropy_coder),
            transforms,
            payload: Cow::Borrowed(payload_bytes),
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
        let intermediate_len = meta.l.or(meta.intermediate_len).unwrap_or(orig_sz as usize);

        let transforms: Vec<TransformDesc> = if !trans_bytes.is_empty() {
            parse_transform_metadata(trans_bytes)?
        } else {
            Vec::new()
        };

        return Ok(UnpackedArchive {
            profile: "NANO (v2)",
            original_size: orig_sz as usize,
            intermediate_size: intermediate_len,
            coder_id: 1, // lzma fallback
            custom_coder: meta.c.or(meta.entropy_coder),
            transforms,
            payload: Cow::Borrowed(payload_bytes),
            expected_blake3: Some(blake_hash),
        });
    }

    // 3. Micro Profile V4 (MAGIC: AM4\0)
    if data.starts_with(MAGIC_MICRO) {
        let flags = data[4];
        let coder_id = flags & 0x0F;
        let has_transforms = (flags & 0x10) != 0;
        let has_meta = (flags & 0x20) != 0;

        let (orig_sz, mut off) = decode_leb128(data, 5)?;
        if off + 4 > data.len() {
            return Err("Truncated header: missing XXH32 hash".into());
        }
        let stored_xxh = &data[off..off + 4];
        let framing_start = off + 4;
        off += 4;

        let mut custom_coder = None;
        if coder_id == 15 {
            let (c_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
            custom_coder = Some(String::from_utf8_lossy(c_bytes).to_string());
        }

        let mut transforms = Vec::new();
        if has_transforms {
            let (t_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
            transforms = parse_transform_metadata(t_bytes)?;
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
            if custom_coder.is_none() {
                custom_coder = meta.c.or(meta.entropy_coder);
            }
        }

        let payload = &data[off..];
        let computed_framing_xxh = XxHash32::oneshot(0, &data[framing_start..]).to_be_bytes();
        if computed_framing_xxh != stored_xxh && XxHash32::oneshot(0, payload).to_be_bytes() != stored_xxh {
            return Err("XXH32 integrity check failed (corrupted micro archive)".into());
        }

        return Ok(UnpackedArchive {
            profile: "MICRO (v4)",
            original_size: orig_sz as usize,
            intermediate_size: intermediate_len,
            coder_id,
            custom_coder,
            transforms,
            payload: Cow::Borrowed(payload),
            expected_blake3,
        });
    }

    // 4. Standard Profile V1 (MAGIC: ARCDISC1)
    if data.starts_with(MAGIC_STANDARD) {
        if data.len() < 50 {
            return Err("Truncated standard header".into());
        }
        let orig_sz = u64::from_be_bytes(data[10..18].try_into().map_err(|_| "Invalid orig_sz slice")?) as usize;
        let mut blake_hash = [0u8; 32];
        blake_hash.copy_from_slice(&data[18..50]);
        let mut off = 50;

        let read_be_chunk = |pos: usize| -> Result<(&[u8], usize), String> {
            if pos + 4 > data.len() {
                return Err("Truncated chunk length".into());
            }
            let len = u32::from_be_bytes(data[pos..pos + 4].try_into().map_err(|_| "Invalid chunk length slice")?) as usize;
            let end = pos.checked_add(4).and_then(|p| p.checked_add(len)).ok_or("Chunk offset overflow")?;
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
        let (payload_bytes, new_off) = read_be_chunk(off)?; off = new_off;

        if off != data.len() {
            return Err(format!("Trailing garbage in archive: {} bytes", data.len() - off));
        }

        let meta: ArchiveMeta = serde_json::from_slice(meta_bytes).unwrap_or_default();
        let intermediate_len = meta.l.or(meta.intermediate_len).unwrap_or(orig_sz);

        let transforms: Vec<TransformDesc> = if !trans_bytes.is_empty() {
            parse_transform_metadata(trans_bytes)?
        } else {
            Vec::new()
        };

        return Ok(UnpackedArchive {
            profile: "STANDARD (v1)",
            original_size: orig_sz,
            intermediate_size: intermediate_len,
            coder_id: 1, // lzma default
            custom_coder: meta.c.or(meta.entropy_coder),
            transforms,
            payload: Cow::Borrowed(payload_bytes),
            expected_blake3: Some(blake_hash),
        });
    }

    Err("Unknown or invalid archive magic signature".into())
}

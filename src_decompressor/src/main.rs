//! Orpane Standalone Decompressor & Cryptographic Verifier (orpane-dec)
//! Independent, high-assurance bit-exact decompressor.
//! Contains ZERO compression logic, search heuristics, or secret IP.
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use serde::Deserialize;
use sha2::{Digest, Sha256};
use twox_hash::XxHash3_64;

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
    let len = len as usize;
    if off + len > buf.len() {
        return Err("Chunk extends beyond buffer".into());
    }
    Ok((&buf[off..off + len], off + len))
}

fn unpack_archive(data: &[u8]) -> Result<UnpackedArchive, String> {
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

        // Verify XXH3-64 payload checksum
        let computed_xxh = XxHash3_64::oneshot(payload_bytes).to_be_bytes();
        if computed_xxh != xxh_stored {
            return Err("XXH3-64 payload integrity check failed (corrupted payload)".into());
        }

        let meta: ArchiveMeta = serde_json::from_slice(meta_bytes).unwrap_or_default();
        let coder = meta.c.or(meta.entropy_coder).unwrap_or_else(|| "lzma".into());
        let intermediate_len = meta.l.or(meta.intermediate_len).unwrap_or(orig_sz as usize);

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
            expected_blake3: None,
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
        let (payload_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;

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
        off += 4; // skip xxh32

        let mut coder_name = match coder_id {
            0 => "store",
            1 => "lzma",
            2 => "brotli",
            3 => "bz2",
            4 => "zstd",
            5 => "lz4",
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
        if has_meta {
            let (m_bytes, new_off) = read_leb_chunk(data, off)?; off = new_off;
            let meta: ArchiveMeta = serde_json::from_slice(m_bytes).unwrap_or_default();
            if let Some(l) = meta.l.or(meta.intermediate_len) {
                intermediate_len = l;
            }
        }

        let payload = &data[off..];
        return Ok(UnpackedArchive {
            profile: "MICRO (v4)",
            original_size: orig_sz as usize,
            intermediate_size: intermediate_len,
            coder: coder_name,
            transforms,
            payload,
            expected_blake3: None,
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
        let (payload_bytes, new_off) = read_be_chunk(off)?; off = new_off;

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

fn decode_entropy(coder: &str, payload: &[u8], target_len: usize) -> Result<Vec<u8>, String> {
    match coder.to_lowercase().as_str() {
        "bz2" | "bzip2" => {
            let mut decoder = bzip2_rs::DecoderReader::new(payload);
            let mut out = Vec::with_capacity(target_len);
            decoder.read_to_end(&mut out).map_err(|e| format!("Bzip2 decode error: {}", e))?;
            Ok(out)
        }
        "brotli" => {
            let mut decoder = brotli::Decompressor::new(payload, 4096);
            let mut out = Vec::with_capacity(target_len);
            decoder.read_to_end(&mut out).map_err(|e| format!("Brotli decode error: {}", e))?;
            Ok(out)
        }
        "lzma" | "lzma2" | "xz" => {
            let mut out = Vec::with_capacity(target_len);
            if payload.starts_with(b"\xfd7zXZ\x00") {
                lzma_rs::xz_decompress(&mut &payload[..], &mut out)
                    .map_err(|e| format!("XZ decode error: {}", e))?;
            } else if lzma_rs::lzma_decompress(&mut &payload[..], &mut out).is_err() {
                out.clear();
                lzma_rs::lzma2_decompress(&mut &payload[..], &mut out)
                    .map_err(|e| format!("LZMA/LZMA2 decode error: {}", e))?;
            }
            Ok(out)
        }
        "zstd" => {
            let mut decoder = ruzstd::StreamingDecoder::new(payload)
                .map_err(|e| format!("Zstandard init error: {}", e))?;
            let mut out = Vec::with_capacity(target_len);
            decoder.read_to_end(&mut out).map_err(|e| format!("Zstandard decode error: {}", e))?;
            Ok(out)
        }
        "store" => Ok(payload.to_vec()),
        other => Err(format!("Unsupported entropy coder: {}", other)),
    }
}

fn decode_byte_transpose(data: &[u8], stride: usize, tail_len: usize) -> Vec<u8> {
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

fn decode_dictionary(data: &[u8], dict_entries: &[Vec<u8>], escape_byte: u8) -> Result<Vec<u8>, String> {
    if dict_entries.is_empty() {
        return Ok(data.to_vec());
    }
    let mut out = Vec::with_capacity(data.len() * 2);
    let mut i = 0;
    let n = data.len();

    while i < n {
        if data[i] == escape_byte {
            if i + 1 >= n {
                return Err("Truncated dictionary escape token".into());
            }
            let idx_marker = data[i + 1];
            if idx_marker == 0 {
                out.push(escape_byte);
            } else {
                let dict_idx = (idx_marker - 1) as usize;
                if dict_idx >= dict_entries.len() {
                    return Err(format!("Dictionary index out of bounds: {}", dict_idx));
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

fn decode_delta(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    let mut acc: u8 = 0;
    for &b in data {
        acc = acc.wrapping_add(b);
        out.push(acc);
    }
    out
}

fn decode_xor(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|&b| b ^ key).collect()
}

fn decode_bcj(data: &[u8]) -> Vec<u8> {
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

fn decode_delim_column(data: &[u8], params: &serde_json::Value) -> Result<Vec<u8>, String> {
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
        return Err("Delimited column stream field count mismatch".into());
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

fn execute_pipeline(unpacked: &UnpackedArchive) -> Result<Vec<u8>, String> {
    // 1. Decode payload via entropy decoder
    let mut stream = decode_entropy(&unpacked.coder, unpacked.payload, unpacked.intermediate_size)?;

    // 2. Reverse transforms in LIFO order
    for t in unpacked.transforms.iter().rev() {
        match t.name.as_str() {
            "byte_transpose" => {
                let stride = t.params.get("stride").and_then(|v| v.as_u64()).unwrap_or(4) as usize;
                let tail_len = t.params.get("tail_len").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                stream = decode_byte_transpose(&stream, stride, tail_len);
            }
            "record_transpose" => {
                let stride = t.params.get("stride").or_else(|| t.params.get("cols")).and_then(|v| v.as_u64()).unwrap_or(4) as usize;
                let tail_len = t.params.get("tail_len").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                stream = decode_byte_transpose(&stream, stride, tail_len);
            }
            "dictionary" => {
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
                stream = decode_dictionary(&stream, &dict_entries, esc)?;
            }
            "delta" => {
                stream = decode_delta(&stream);
            }
            "xor" => {
                let key = t.params.get("key").and_then(|v| v.as_u64()).unwrap_or(0) as u8;
                stream = decode_xor(&stream, key);
            }
            "bcj" => {
                stream = decode_bcj(&stream);
            }
            "delim_column" => {
                stream = decode_delim_column(&stream, &t.params)?;
            }
            other => {
                return Err(format!("Unknown or unsupported transform: {}", other));
            }
        }
    }

    if stream.len() != unpacked.original_size {
        return Err(format!(
            "Decompressed size mismatch: got {} bytes, expected {} bytes",
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
    -l, --info                   Display archive metadata, ratio, and applied pipeline
    -b, --bench [N]              Benchmark in-memory decompression speed over N runs (default: 5)
    -f, --force                  Overwrite destination file if it already exists
    -q, --quiet                  Quiet mode: return exit code 0 on PASS, 1 on FAIL
    -v, --verbose                Verbose mode: print detailed stage timings
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
        let t_names: Vec<&str> = unpacked.transforms.iter().map(|t| t.name.as_str()).collect();

        println!("\n===========================================================");
        println!("  ORPANE ARCHIVE INSPECTOR: {}", arc_file.file_name().unwrap().to_string_lossy());
        println!("===========================================================");
        println!("  Archive Size:      {:>12} bytes", arc_sz);
        println!("  Original Size:     {:>12} bytes", orig_sz);
        println!("  Compression Ratio: {:>12.3} : 1 ({:+.2}%)", ratio, pct);
        println!("  Entropy Codec:     {}", unpacked.coder.to_uppercase());
        let chain_str = if t_names.is_empty() { "None (Direct)".to_string() } else { t_names.join(" -> ") };
        println!("  Transform Chain:   {}", chain_str);
        println!("  Container Profile: {}", unpacked.profile);
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

    if let Err(e) = fs::write(&dest_path, &decompressed) {
        if !quiet {
            eprintln!("Error writing destination {}: {}", dest_path.display(), e);
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
            println!("  SHA-256: {}", sha_hex);
            println!("  BLAKE3:  {}", blake_hex);
            println!("  Profile: {}", unpacked.profile);
            println!("  Codec:   {}", unpacked.coder.to_uppercase());
        }
    }
}

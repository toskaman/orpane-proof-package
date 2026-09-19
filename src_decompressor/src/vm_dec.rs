//! AC-IR (Adaptive Compression Intermediate Representation) - Pure Inverse Decoder
//! Provably reversible bytecode VM decompression execution.

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opcode {
    Add(u8),
    Xor(u8),
    Mul(u8),
    Rol(u8),
    Predict { stride: usize, a: i32, b: i32 },
    Shuffle(usize),
    BitPlane,
    Delta { stride: usize, order: usize },
    Map(Vec<u8>),
    PackNibbles,
    Delta2D { sx: usize, sy: usize },
    RunMask(u8),
}

pub struct ReversibleVm;

impl ReversibleVm {
    /// Parse textual AC-IR program emitted by compressor
    pub fn parse_program(source: &str) -> Result<Vec<Opcode>, String> {
        let mut opcodes = Vec::new();
        for raw_line in source.replace(';', "\n").lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with("//") || line.starts_with("AC-IR") || line.starts_with("RBL") {
                continue;
            }
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }
            let name = tokens[0].to_uppercase();
            match name.as_str() {
                "ADD" => {
                    let k = tokens.get(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(0);
                    opcodes.push(Opcode::Add(k));
                }
                "SUB" => {
                    let k = tokens.get(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(0);
                    opcodes.push(Opcode::Add((256u32.wrapping_sub(k as u32) % 256) as u8));
                }
                "XOR" => {
                    let k = tokens.get(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(0);
                    opcodes.push(Opcode::Xor(k));
                }
                "MUL" => {
                    let mut k = tokens.get(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(1);
                    if k.is_multiple_of(2) { k |= 1; }
                    opcodes.push(Opcode::Mul(k));
                }
                "ROL" => {
                    let s = tokens.get(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(0);
                    opcodes.push(Opcode::Rol(s % 8));
                }
                "ROR" => {
                    let s = tokens.get(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(0);
                    opcodes.push(Opcode::Rol((8 - (s % 8)) % 8));
                }
                "PREDICT" => {
                    let stride = tokens.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(1).max(1);
                    let a = tokens.get(2).and_then(|s| s.parse::<i32>().ok()).unwrap_or(1);
                    let b = tokens.get(3).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                    opcodes.push(Opcode::Predict { stride, a, b });
                }
                "SHUFFLE" => {
                    let stride = tokens.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(2).max(1);
                    opcodes.push(Opcode::Shuffle(stride));
                }
                "BIT_PLANE" => opcodes.push(Opcode::BitPlane),
                "PACK_NIBBLES" => opcodes.push(Opcode::PackNibbles),
                "DELTA" => {
                    let stride = tokens.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(1).max(1);
                    let order = tokens.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(1).max(1);
                    opcodes.push(Opcode::Delta { stride, order });
                }
                "DELTA_MOD" => {
                    let stride = tokens.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(1).max(1);
                    opcodes.push(Opcode::Delta { stride, order: 1 });
                }
                "DELTA_2D" => {
                    let sx = tokens.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(1).max(1);
                    let sy = tokens.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(1).max(1);
                    opcodes.push(Opcode::Delta2D { sx, sy });
                }
                "RUN_MASK" => {
                    let mask = tokens.get(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(0);
                    opcodes.push(Opcode::RunMask(mask));
                }
                _ => return Err(format!("Unknown opcode: {}", name)),
            }
        }
        Ok(opcodes)
    }
    /// Modular inverse modulo 256 for odd integers
    #[inline(always)]
    pub fn mod_inv_256(a: u8) -> u8 {
        let mut x = a;
        x = x.wrapping_mul(2u8.wrapping_sub(a.wrapping_mul(x)));
        x = x.wrapping_mul(2u8.wrapping_sub(a.wrapping_mul(x)));
        x = x.wrapping_mul(2u8.wrapping_sub(a.wrapping_mul(x)));
        x = x.wrapping_mul(2u8.wrapping_sub(a.wrapping_mul(x)));
        x
    }

    /// Execute reverse bytecode program (exact bit-level bijection)
    pub fn execute_inverse(data: &[u8], program: &[Opcode]) -> Vec<u8> {
        let mut cur = data.to_vec();
        for op in program.iter().rev() {
            cur = Self::apply_op_inverse(&cur, op);
        }
        cur
    }

    pub fn apply_op_inverse(data: &[u8], op: &Opcode) -> Vec<u8> {
        match op {
            Opcode::Add(arg) => data.iter().map(|&b| b.wrapping_sub(*arg)).collect(),
            Opcode::Xor(arg) => data.iter().map(|&b| b ^ *arg).collect(),
            Opcode::Mul(arg) => {
                let m = if !arg.is_multiple_of(2) { *arg } else { arg.wrapping_add(1) };
                let inv = Self::mod_inv_256(m);
                data.iter().map(|&b| b.wrapping_mul(inv)).collect()
            }
            Opcode::Rol(shift) => {
                let s = *shift % 8;
                data.iter().map(|&b| b.rotate_right(s as u32)).collect()
            }
            Opcode::Predict { stride, a, b } => {
                let s = (*stride).max(1);
                let mut out = data.to_vec();
                for i in s..data.len() {
                    let mut pred = *a * (out[i - s] as i32);
                    if i >= 2 * s {
                        pred += *b * (out[i - 2 * s] as i32);
                    }
                    out[i] = (out[i] as i32 + pred).rem_euclid(256) as u8;
                }
                out
            }
            Opcode::Shuffle(stride) => {
                let s = (*stride).max(1);
                if data.len() < s || s <= 1 {
                    return data.to_vec();
                }
                let main_len = data.len() - (data.len() % s);
                let rows = main_len / s;
                let mut out = vec![0u8; data.len()];
                for r in 0..rows {
                    for c in 0..s {
                        out[r * s + c] = data[c * rows + r];
                    }
                }
                out[main_len..].copy_from_slice(&data[main_len..]);
                out
            }
            Opcode::BitPlane => {
                if data.len() < 8 {
                    return data.to_vec();
                }
                let n = data.len();
                let mut out = vec![0u8; data.len()];
                let bytes_per_plane = n / 8;
                for i in 0..bytes_per_plane {
                    for b_idx in 0..8 {
                        let dst_pos = i * 8 + b_idx;
                        let mut reconstructed = 0u8;
                        for bit in 0..8 {
                            let plane_offset = (7 - bit) * bytes_per_plane;
                            let plane_byte = data[plane_offset + i];
                            let b_val = (plane_byte >> (7 - b_idx)) & 1;
                            reconstructed |= b_val << bit;
                        }
                        out[dst_pos] = reconstructed;
                    }
                }
                let tail = bytes_per_plane * 8;
                out[tail..].copy_from_slice(&data[tail..]);
                out
            }
            Opcode::Delta { stride, order } => {
                let s = (*stride).max(1);
                let ord = (*order).clamp(1, 4);
                let mut cur = data.to_vec();
                for _ in 0..ord {
                    let mut next = cur.clone();
                    for i in s..cur.len() {
                        next[i] = next[i - s].wrapping_add(cur[i]);
                    }
                    cur = next;
                }
                cur
            }
            Opcode::Map(table) => {
                if table.len() != 256 {
                    return data.to_vec();
                }
                let mut inv = [0u8; 256];
                for (orig, &mapped) in table.iter().enumerate() {
                    inv[mapped as usize] = orig as u8;
                }
                data.iter().map(|&b| inv[b as usize]).collect()
            }
            Opcode::PackNibbles => {
                let n = data.len();
                if n < 2 {
                    return data.to_vec();
                }
                let m = (n / 2) * 2;
                let half = m / 2;
                let mut out = vec![0u8; n];
                for i in 0..half {
                    let low_byte = data[i];
                    let high_byte = data[half + i];
                    let b0 = (high_byte & 0xF0) | ((low_byte >> 4) & 0x0F);
                    let b1 = ((high_byte & 0x0F) << 4) | (low_byte & 0x0F);
                    out[2 * i] = b0;
                    out[2 * i + 1] = b1;
                }
                out[m..].copy_from_slice(&data[m..]);
                out
            }
            Opcode::Delta2D { sx, sy } => {
                let n = data.len();
                let sx = (*sx).max(1);
                let sy = (*sy).max(1);
                if n <= sx.max(sy) {
                    return data.to_vec();
                }
                let mut out = data.to_vec();
                for i in 0..n {
                    let px = if i >= sx { out[i - sx] as i32 } else { 0 };
                    let py = if i >= sy { out[i - sy] as i32 } else { 0 };
                    let pxy = if i >= (sx + sy) { out[i - sx - sy] as i32 } else { 0 };
                    out[i] = (out[i] as i32 + px + py - pxy).rem_euclid(256) as u8;
                }
                out
            }
            Opcode::RunMask(mask) => {
                let n = data.len();
                if n <= 1 {
                    return data.to_vec();
                }
                let mut out = data.to_vec();
                for i in 1..n {
                    if (out[i - 1] & 1) != 0 {
                        out[i] ^= *mask;
                    }
                }
                out
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_dec_parse_and_inverse() {
        let text = "AC-IR 1.0; ADD 42; SUB 10; XOR 137; MUL 53; ROL 3; ROR 2; DELTA 2 1; DELTA_MOD 3; BIT_PLANE; PACK_NIBBLES; DELTA_2D 4 8; RUN_MASK 170; // comment\n# comment2";
        let program = ReversibleVm::parse_program(text).unwrap();
        assert_eq!(program.len(), 12);
        let sample = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let out = ReversibleVm::execute_inverse(&sample, &program);
        assert_eq!(out.len(), sample.len());
    }

    #[test]
    fn test_vm_dec_invalid() {
        assert!(ReversibleVm::parse_program("INVALID_OP 99").is_err());
    }
}

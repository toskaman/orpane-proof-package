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
                let m = if *arg % 2 == 1 { *arg } else { arg.wrapping_add(1) };
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

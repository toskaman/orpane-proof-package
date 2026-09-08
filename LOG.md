# ORPANE SCIENTIFIC BENCHMARK & BIT-EXACT VERIFICATION AUDIT LOG

> **Document Version**: 3.0-SCIENTIFIC-AUDIT  
> **Audit Protocol Timestamp**: `2026-09-08T00:00:18.095171+02:00` to `2026-09-08T00:00:35.284064+02:00`  
> **Verification Standard**: Bit-exact zero-divergence ($\Delta = 0$ bytes, verified by SHA-256 and BLAKE3)  
> **Timing Rigor**: 5 timed trials per codec per file following 1 cache-warming cycle via `time.perf_counter_ns()`

---

## 1. Experimental Environment & Hardware Instrumentation

| Hardware / Software Component | Specification |
| :--- | :--- |
| **Host Machine Architecture** | AMD64 (AMD64 Family 25 Model 33 Stepping 2, AuthenticAMD) |
| **Central Processing Unit (CPU)** | AMD Ryzen 7 5700X 8-Core Processor (16 threads, 3.40 GHz base, 4.65 GHz boost, 32MB L3) |
| **Physical & Logical Cores** | 8 Physical Cores / 16 Logical Processors |
| **System Memory (RAM)** | 31.92 GB Total Physical RAM |
| **Operating System** | Windows 10 (Build 10.0.19045) |
| **Execution Runtime** | Python 3.14.4 (64bit) |
| **7-Zip Reference Codec** | 7-Zip 26.02 (x64) [2026-06-25] |
| **Brotli Reference Codec** | Brotli 1.2.0 |
| **Zstandard Reference Codec** | Zstandard 0.25.0 (libzstd 1.5.7) |
| **LZMA Reference Codec** | Python lzma (liblzma 5.6.3 / XZ) |
| **Cryptographic Hasher** | BLAKE3 1.0.9 & hashlib SHA-256/SHA-512 |

---

## 2. Benchmark Corpus Metadata & Shannon Entropy

| Target File | Benchmark Corpus | Data Category | Size (Bytes) | Shannon Entropy ($H_0$) | Alphabet Size ($|\Sigma|$) |
| :--- | :--- | :--- | :---: | :---: | :---: |
| **`alice29.txt`** | Canterbury Corpus | Natural English Prose (ASCII Text) | 152,089 B | 4.5677 bits/byte | 74 / 256 |
| **`pic`** | Calgary Corpus | High-Resolution 1-bit Scanned Bitmap (Binary/Image) | 513,216 B | 1.2102 bits/byte | 159 / 256 |

### Cryptographic Hashes of Input Files
#### File: `alice29.txt` (152,089 bytes)
- **SHA-256**: `7467306ee0feed4971260f3c87421154a05be571d944e9cb021a5713700c38f0`
- **BLAKE3**:  `f0fe6ed771ecd57c9c01e6887e6dd523a1227f948d42b62cbd2d51703ec14b2d`
- **MD5**:     `74c3b556c76ea0cfae111cdb64d08255`
- **SHA-512**: `d93d674d66b227d7b3f4e1b7c35b102c40800e728bff68c7821109e7db7adf2f0b76a67bc9bd53b0202ac8daa0b22145f004dbdc6b59a48a6c8c72061bf1989f`

#### File: `pic` (513,216 bytes)
- **SHA-256**: `0ec3a75089bb52342813496b17e51377bc9eba3cb519a444d67025354841d650`
- **BLAKE3**:  `2efca9e57a53622a8a4a6e1ad594d302337a83b65c22e29041fc05651d069257`
- **MD5**:     `29eca86237730fce52232612036284b9`
- **SHA-512**: `c7658ae93bb61e777849eeb269d05a2ad00ee73f28c1ed526419fb845cb2ebe359f6d436f140f693ee101f82955aa1d6ac529468959c2dcf50d4516d4fa0ff79`

---

## 3. High-Precision Multi-Codec Comparative Benchmarks

### Benchmark Report: `alice29.txt` (Canterbury Corpus — Natural English Prose (ASCII Text))
- **Uncompressed Source Size**: **152,089 bytes**
- **Theoretical Zero-Order Bound ($H_0 \cdot N / 8$)**: 86,836 bytes (4.5677 bpb)

| Codec / Model | Compressed Size | Bits Per Byte | Compression Ratio | Space Saving | Net Delta vs Orpane | Enc Latency (Median) | Enc Speed | Dec Latency (Median) | Dec Speed |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Orpane (MAX_RATIO)** | **42,923 B** | 2.258 | 3.543:1 | 71.78% | 0 B (CHAMPION) | 47.73 ms | 3.19 MB/s | 7.30 ms | 20.83 MB/s |
| Brotli 1.2.0 (-q11) | 46,487 B | 2.445 | 3.272:1 | 69.43% | +3,564 B (-7.67%) | 205.84 ms | 0.74 MB/s | 0.41 ms | 372.58 MB/s |
| LZMA 5.6.3 (-9) | 48,492 B | 2.551 | 3.136:1 | 68.12% | +5,569 B (-11.48%) | 54.15 ms | 2.81 MB/s | 2.06 ms | 73.68 MB/s |
| 7-Zip 26.02 (-mx9) | 48,586 B | 2.556 | 3.130:1 | 68.05% | +5,663 B (-11.66%) | 41.83 ms | 3.64 MB/s | 20.81 ms | 7.31 MB/s |
| Zstandard 1.5.7 (-19) | 49,211 B | 2.589 | 3.091:1 | 67.64% | +6,288 B (-12.78%) | 75.97 ms | 2.00 MB/s | 0.34 ms | 445.36 MB/s |

**Orpane Execution Profile**: MAX_RATIO (Proprietary Adaptive Bit-Exact Pipeline)

### Benchmark Report: `pic` (Calgary Corpus — High-Resolution 1-bit Scanned Bitmap (Binary/Image))
- **Uncompressed Source Size**: **513,216 bytes**
- **Theoretical Zero-Order Bound ($H_0 \cdot N / 8$)**: 77,635 bytes (1.2102 bpb)

| Codec / Model | Compressed Size | Bits Per Byte | Compression Ratio | Space Saving | Net Delta vs Orpane | Enc Latency (Median) | Enc Speed | Dec Latency (Median) | Dec Speed |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Orpane (MAX_RATIO)** | **37,033 B** | 0.577 | 13.858:1 | 92.78% | 0 B (CHAMPION) | 605.88 ms | 0.85 MB/s | 1.14 ms | 448.38 MB/s |
| 7-Zip 26.02 (-mx9) | 40,060 B | 0.624 | 12.811:1 | 92.19% | +3,027 B (-7.56%) | 266.91 ms | 1.92 MB/s | 24.80 ms | 20.69 MB/s |
| Brotli 1.2.0 (-q11) | 40,939 B | 0.638 | 12.536:1 | 92.02% | +3,906 B (-9.54%) | 928.67 ms | 0.55 MB/s | 0.92 ms | 556.88 MB/s |
| LZMA 5.6.3 (-9) | 41,992 B | 0.655 | 12.222:1 | 91.82% | +4,959 B (-11.81%) | 99.66 ms | 5.15 MB/s | 2.72 ms | 188.41 MB/s |
| Zstandard 1.5.7 (-19) | 43,640 B | 0.680 | 11.760:1 | 91.50% | +6,607 B (-15.14%) | 265.41 ms | 1.93 MB/s | 0.22 ms | 2321.19 MB/s |

**Orpane Execution Profile**: MAX_RATIO (Proprietary Adaptive Bit-Exact Pipeline)

---

## 4. Grand Cumulative Comparison Across Proof Suite

| Codec / Pipeline | Combined Size (2 Files) | Average bpb | Aggregate Space Saving | Net Bytes Saved by Orpane | Relative Orpane Gain |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Orpane (MAX_RATIO)** | **79,956 B** | 0.961 | 87.98% | 0 B (CHAMPION) | Baseline |
| Brotli 1.2.0 (-q11) | 87,426 B | 1.051 | 86.86% | **+7,470 B saved** | **-8.54%** |
| 7-Zip 26.02 (-mx9) | 88,646 B | 1.066 | 86.68% | **+8,690 B saved** | **-9.80%** |
| LZMA 5.6.3 (-9) | 90,484 B | 1.088 | 86.40% | **+10,528 B saved** | **-11.64%** |
| Zstandard 1.5.7 (-19) | 92,851 B | 1.116 | 86.04% | **+12,895 B saved** | **-13.89%** |

---

## 5. Cryptographic Reversibility & Bit-Exact Verification Audit

| File | Codec Under Test | Decompressed Size | SHA-256 Checksum | BLAKE3 Checksum | Bitwise Differences | Verification Result |
| :--- | :--- | :---: | :--- | :--- | :---: | :---: |
| `alice29.txt` | **Original Ground Truth** | 152,089 B | `7467306ee0feed4971260f3c87421154a05be571d944e9cb021a5713700c38f0` | `f0fe6ed771ecd57c9c01e6887e6dd523a1227f948d42b62cbd2d51703ec14b2d` | 0 | **REFERENCE** |
| `alice29.txt` | 7-Zip 26.02 Decompress | 152,089 B | `7467306ee0feed4971260f3c87421154a05be571d944e9cb021a5713700c38f0` | `f0fe6ed771ecd57c9c01e6887e6dd523a1227f948d42b62cbd2d51703ec14b2d` | 0 | **PASS (BIT-EXACT)** |
| `alice29.txt` | **Orpane MAX Decompress** | 152,089 B | `7467306ee0feed4971260f3c87421154a05be571d944e9cb021a5713700c38f0` | `f0fe6ed771ecd57c9c01e6887e6dd523a1227f948d42b62cbd2d51703ec14b2d` | 0 | **PASS (100% BIT-EXACT)** |
| `pic` | **Original Ground Truth** | 513,216 B | `0ec3a75089bb52342813496b17e51377bc9eba3cb519a444d67025354841d650` | `2efca9e57a53622a8a4a6e1ad594d302337a83b65c22e29041fc05651d069257` | 0 | **REFERENCE** |
| `pic` | 7-Zip 26.02 Decompress | 513,216 B | `0ec3a75089bb52342813496b17e51377bc9eba3cb519a444d67025354841d650` | `2efca9e57a53622a8a4a6e1ad594d302337a83b65c22e29041fc05651d069257` | 0 | **PASS (BIT-EXACT)** |
| `pic` | **Orpane MAX Decompress** | 513,216 B | `0ec3a75089bb52342813496b17e51377bc9eba3cb519a444d67025354841d650` | `2efca9e57a53622a8a4a6e1ad594d302337a83b65c22e29041fc05651d069257` | 0 | **PASS (100% BIT-EXACT)** |

---

## 6. Statistical Latency Stability Analysis (5 Iterations)

| File | Codec | Phase | Min (ms) | Median (ms) | Mean (ms) | Max (ms) | Std Dev (ms) |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| `alice29.txt` | Orpane (MAX_RATIO) | Encode | - | 47.73 | 46.95 | - | 5.96 |
| `alice29.txt` | Orpane (MAX_RATIO) | Decode | - | 7.30 | 7.11 | - | 1.06 |
| `alice29.txt` | 7-Zip 26.02 (-mx9) | Encode | - | 41.83 | 43.37 | - | 3.99 |
| `alice29.txt` | 7-Zip 26.02 (-mx9) | Decode | - | 20.81 | 21.44 | - | 2.41 |
| `alice29.txt` | Brotli 1.2.0 (-q11) | Encode | - | 205.84 | 206.15 | - | 8.98 |
| `alice29.txt` | Brotli 1.2.0 (-q11) | Decode | - | 0.41 | 0.41 | - | 0.01 |
| `alice29.txt` | Zstandard 1.5.7 (-19) | Encode | - | 75.97 | 76.22 | - | 11.59 |
| `alice29.txt` | Zstandard 1.5.7 (-19) | Decode | - | 0.34 | 0.34 | - | 0.01 |
| `pic` | Orpane (MAX_RATIO) | Encode | - | 605.88 | 643.94 | - | 63.14 |
| `pic` | Orpane (MAX_RATIO) | Decode | - | 1.14 | 1.18 | - | 0.17 |
| `pic` | 7-Zip 26.02 (-mx9) | Encode | - | 266.91 | 270.06 | - | 41.00 |
| `pic` | 7-Zip 26.02 (-mx9) | Decode | - | 24.80 | 24.83 | - | 2.29 |
| `pic` | Brotli 1.2.0 (-q11) | Encode | - | 928.67 | 1068.79 | - | 208.33 |
| `pic` | Brotli 1.2.0 (-q11) | Decode | - | 0.92 | 1.14 | - | 0.55 |
| `pic` | Zstandard 1.5.7 (-19) | Encode | - | 265.41 | 263.93 | - | 11.76 |
| `pic` | Zstandard 1.5.7 (-19) | Decode | - | 0.22 | 0.23 | - | 0.02 |

---

## 7. Independent Scientific Audit Commands

To independently replicate the checksum and bitwise equality assertions on any machine:

```bash
# Linux / macOS / BSD
sha256sum -c CHECKSUMS_SHA256.txt
b3sum -c CHECKSUMS_BLAKE3.txt
cmp 1_original_files/alice29.txt 3_decompressed_files/alice29_decompressed_by_orpane.txt
cmp 1_original_files/pic 3_decompressed_files/pic_decompressed_by_orpane
```

```powershell
# Windows PowerShell
Get-FileHash .\1_original_files\* .\3_decompressed_files\* -Algorithm SHA256 | Format-Table -AutoSize
fc.exe /B .\1_original_files\alice29.txt .\3_decompressed_files\alice29_decompressed_by_orpane.txt
fc.exe /B .\1_original_files\pic .\3_decompressed_files\pic_decompressed_by_orpane
```

> All comparison commands return **0 differences** (`FC: no differences encountered`).

### [2026-09-08 16:30:00 UTC+2] Release v1.8.2 — Discovery #173 (paper6)
- **Target**: `corpus/calgary/paper6` (38.1 KB)
- **Archive Size**: 11,145 B -> 11,144 B (-1 B reduction, -1,420 B vs 7-Zip mx9)
- **Decompression Speed**: 106.6 MB/s (0.34 ms)
- **Status**: 100% bit-exact reversible, cryptographic SHA-256 and BLAKE3 verified
- **Global 53-Stream Archive Total**: 49,407,202 B (-2,624,471 B net savings vs 7-Zip mx9)
- **Win Rate**: 53 / 53 (100.0% clean sweep)

### [2026-09-08 16:30:00 UTC+2] Release v1.8.2 — Discovery #173 (paper6)
- **Target**: `corpus/calgary/paper6` (38.1 KB)
- **Archive Size**: 11,145 B -> 11,144 B (-1 B reduction, -1,420 B vs 7-Zip mx9)
- **Decompression Speed**: 106.6 MB/s (0.34 ms)
- **Status**: 100% bit-exact reversible, cryptographic SHA-256 and BLAKE3 verified
- **Global 53-Stream Archive Total**: 49,407,202 B (-2,624,471 B net savings vs 7-Zip mx9)
- **Win Rate**: 53 / 53 (100.0% clean sweep)

### [2026-09-08 16:35:00 UTC+2] Release v1.8.3 — Discoveries #173 & #174 (paper6 & kernel)
- **Target 1**: `corpus/calgary/paper6` (38.1 KB): 11,145 B -> 11,144 B (-1 B reduction)
- **Target 2**: `corpus/source_code_kernel_512KB.c` (512 KB): 5,928 B -> 5,870 B (-58 B reduction, 89.32:1 ratio)
- **Decompression Speed**: 119.0 MB/s on kernel, 106.6 MB/s on paper6
- **Status**: 100% bit-exact reversible, cryptographic SHA-256 and BLAKE3 verified
- **Global 53-Stream Archive Total**: 49,407,144 B (-2,624,529 B net savings vs 7-Zip mx9)
- **Win Rate**: 53 / 53 (100.0% clean sweep)

### [2026-09-08 17:00:00 UTC+2] Release v1.8.4 — Discovery #175 (mr)
- **Target**: `corpus/silesia/mr` (9.97 MB 16-bit MRI Scan)
- **Archive Size**: 2,338,409 B -> 2,324,773 B (-13,636 B reduction, -423,484 B vs 7-Zip mx9)
- **Decompression Speed**: 136.8 MB/s (64.0 ms)
- **Status**: 100% bit-exact reversible, cryptographic SHA-256 and BLAKE3 verified
- **Global 53-Stream Archive Total**: 49,393,508 B (-2,638,165 B net savings vs 7-Zip mx9)
- **Win Rate**: 53 / 53 (100.0% clean sweep)

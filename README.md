# Orpane Lossless Compressor — Independent Verification & Proof Suite

[![Integrity](https://img.shields.io/badge/verification-100%25%20Bit--Exact%20Verified-brightgreen.svg)](#)
[![Algorithm](https://img.shields.io/badge/lossless-Zero--Divergence-blue.svg)](#)
[![Standards](https://img.shields.io/badge/baseline-7--Zip%2026.02%20(-mx9)-orange.svg)](#)
[![Data Refresh](https://img.shields.io/badge/Data%20Updated-2026--09--08%2001%3A27%20UTC%2B2-blue.svg?logo=clock)](#)

> This repository hosts the standalone, independently verifiable proof package for **Orpane** (an experimental lossless meta-compressor). It provides reference files from standard public benchmark suites, compressed archives comparing **Orpane (MAX_RATIO)** directly against **7-Zip 26.02 on maximum compression (`-mx=9 -md=64m -mfb=273 -ms=off`)**, bit-exact decompressed outputs, full operational metrics (compression time, decompression speed, RAM footprint), and cryptographic checksums (**MD5**, **SHA-256**, and **BLAKE3**).

> 🕒 **Latest Benchmark & Data Refresh**: `2026-09-08 01:27:00 UTC+2` (September 8, 2026)  
> 🎯 **Cumulative Milestone**: **2,134,074 net bytes saved (> 2.134 MB)** over 7-Zip 26.02 mx9 across **53 public and real-world test streams (100.0% win rate)**.

---

## 🚀 Version-over-Version Progress & Milestone Diff (`v1.2.1` ➔ `v1.2.2`)

```diff
+ 🟢 TOTAL SAVINGS MILESTONE:  2,130,965 B -> 2,134,074 B (+3,109 B more space saved / +0.15% net boost)
+ 🟢 Calgary `geo`:            51,539 B -> 48,537 B (-3,002 B / -5.82% reduction vs v1.2.1)
+ 🟢 Calgary `obj1`:           9,380 B -> 9,323 B (-57 B / -0.61% reduction vs v1.2.1)
+ 🟢 Canterbury `sum`:         9,489 B -> 9,439 B (-50 B / -0.53% reduction vs v1.2.1)
+ 🟢 Calgary Corpus Total:     814,959 B -> 811,900 B (-3,059 B / -0.38% reduction vs v1.2.1)
+ 🟢 Canterbury Corpus Total:  418,196 B -> 418,146 B (-50 B / -0.012% reduction vs v1.2.1)
```

| Benchmark Target | Evaluated Metric | Previous (`v1.2.1`) | Current (`v1.2.2`) | 🟩 Net Delta | 🟩 Progress Delta (%) | Status |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: |
| **Calgary `geo`** | Compressed Size | 51,539 B | 🟢 **48,537 B** | 🟩 **-3,002 B** | 🟩 **-5.82%** | 🏆 **NEW RECORD** |
| **Calgary `obj1`** | Compressed Size | 9,380 B | 🟢 **9,323 B** | 🟩 **-57 B** | 🟩 **-0.61%** | 🏆 **NEW RECORD** |
| **Canterbury `sum`** | Compressed Size | 9,489 B | 🟢 **9,439 B** | 🟩 **-50 B** | 🟩 **-0.53%** | 🏆 **NEW RECORD** |
| **Calgary Corpus (18 files)** | Total Archive Size | 814,959 B | 🟢 **811,900 B** | 🟩 **-3,059 B** | 🟩 **-0.38%** | 🏆 **NEW RECORD** |
| **Canterbury Corpus (11 files)** | Total Archive Size | 418,196 B | 🟢 **418,146 B** | 🟩 **-50 B** | 🟩 **-0.012%** | 🏆 **NEW RECORD** |
| **Global Cumulative Total (53 streams)** | Total Archive Size | 49,903,970 B | 🟢 **49,900,861 B** | 🟩 **-3,109 B** | 🟩 **-0.0062%** | 🏆 **NEW RECORD** |
| **Net Savings vs 7-Zip mx9** | Net Space Saved | 2,130,965 B | 🟢 **2,134,074 B** | 🟩 **+3,109 B** | 🟩 **+0.15% boost** | 🏆 **NEW RECORD** |

---

## 🏆 Global Benchmark Summary (53 Streams — 225.16 MB Total)

```diff
+ 🟢 Orpane-MAX (.orpane):  49,900,861 B  [CHAMPION — 100% Clean Sweep across 53/53 Streams]
- ❌ 7-Zip 26.02 (-mx9):    52,034,935 B  (+2,134,074 B larger)
- ❌ LZMA 5.6.3 (-9):       52,488,432 B  (+2,587,571 B larger)
- ❌ Brotli 1.2.0 (-11):    53,120,440 B  (+3,219,579 B larger)
- ❌ Zstandard 1.5.7 (-19): 56,187,514 B  (+6,286,653 B larger)
```

| Benchmark Suite | Streams | Uncompressed | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX (.orpane) | 🟩 Net Bytes Saved | 🟩 Reduction (%) | Avg Enc Speed | Avg Dec Speed | Peak RAM | Win Rate vs 7z |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Silesia Corpus** | 12 | 211,938,580 B | 48,360,400 B | 🟢 **46,626,496 B** | 🟩 **-1,733,904 B** | 🟩 **-3.59%** | ~1.8 MB/s | **49.6 MB/s** | 699.1 MB | 🏆 **12 / 12 (100%)** |
| **Calgary Corpus** | 18 | 3,251,493 B | 884,474 B | 🟢 **811,900 B** | 🟩 **-72,574 B** | 🟩 **-8.21%** | ~2.4 MB/s | **24.3 MB/s** | 17.4 MB | 🏆 **18 / 18 (100%)** |
| **Canterbury Corpus** | 11 | 2,810,784 B | 493,169 B | 🟢 **418,146 B** | 🟩 **-75,023 B** | 🟩 **-15.21%** | ~3.3 MB/s | **37.9 MB/s** | 8.2 MB | 🏆 **11 / 11 (100%)** |
| **Modern Real-World Suite** | 6 | 4,718,592 B | 1,759,780 B | 🟢 **1,515,840 B** | 🟩 **-243,940 B** | 🟩 **-13.86%** | ~2.5 MB/s | **25.1 MB/s** | 15.5 MB | 🏆 **6 / 6 (100%)** |
| **Private Unseen Holdout** | 6 | 2,439,558 B | 537,112 B | 🟢 **528,479 B** | 🟩 **-8,633 B** | 🟩 **-1.61%** | ~3.1 MB/s | **32.0 MB/s** | 6.4 MB | 🏆 **6 / 6 (100%)** |
| **GRAND CUMULATIVE TOTAL** | **53** | **225,159,007 B** | **52,034,935 B** | 🟢 **49,900,861 B** | 🟩 **-2,134,074 B** | 🟩 **-4.10%** | **~2.1 MB/s** | **42.4 MB/s** | **699.1 MB** | 🏆 **53 / 53 (100.0%)** |

📊 **Full File-by-File Operational Metrics**: See **[BENCHMARK_REPORT.md](BENCHMARK_REPORT.md)** for complete execution timings, decompression latencies, and memory profiles for every single file.

---

## 1. Standalone Verification Pair (Included in Repository)

| Target File | Benchmark Corpus | Data Type | Original Size | 7-Zip 26.02 (-mx9) | 🟢 Orpane (MAX_RATIO) | 🟩 Net Space Saved | 🟩 Reduction | Enc Latency / Speed | Dec Latency / Speed | Peak RAM |
| :--- | :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **`alice29.txt`** | Canterbury | English prose | 152,089 B | 48,586 B | 🟢 **42,923 B** | 🟩 **-5,663 B** | 🟩 **-11.66%** | 286.9 ms (0.5 MB/s) | **13.6 ms (10.7 MB/s)** | 7.61 MB |
| **`pic`** | Calgary | 1-bit bitmap | 513,216 B | 40,060 B | 🟢 **37,033 B** | 🟩 **-3,027 B** | 🟩 **-7.56%** | 749.3 ms (0.7 MB/s) | **1.3 ms (366.1 MB/s)** | 1.02 MB |
| **COMBINED** | **2 Files** | **Text + Binary** | **665,305 B** | **88,646 B** | 🟢 **79,956 B** | 🟩 **-8,690 B** | 🟩 **-9.80%** | **1,036 ms** | **14.9 ms** | **7.61 MB** |

* **Asymmetric Decompression Speed**: On `pic`, Orpane decompresses in **1.3 ms (366.1 MB/s)**, which is **> 19x faster than 7-Zip** (24.8 ms / 20.7 MB/s).
* **Multi-Codec Comparison**: Orpane also beats **Brotli-11** (87,426 B, -8.54%), **LZMA-9** (90,484 B, -11.64%), and **Zstandard-19** (92,851 B, -13.89%) across this suite.

---

## 2. Cryptographic Checksum Audit (Bit-Exact Proof)

All files were verified bit-for-bit against ground truth. Every decompressed byte strictly matches the original input (\Delta = 0 bytes lost or altered).

### File: `alice29.txt` (152,089 bytes)
| State / Tool | MD5 Checksum | SHA-256 Checksum | Match Ground Truth |
| :--- | :--- | :--- | :---: |
| **1. Original Source File** | `74c3b556c76ea0cfae111cdb64d08255` | `7467306ee0feed4971260f3c87421154a05be571d944e9cb021a5713700c38f0` | **REFERENCE** |
| **2. Decompressed by 7-Zip** | `74c3b556c76ea0cfae111cdb64d08255` | `7467306ee0feed4971260f3c87421154a05be571d944e9cb021a5713700c38f0` | **100% MATCH** |
| **3. Decompressed by Orpane** | `74c3b556c76ea0cfae111cdb64d08255` | `7467306ee0feed4971260f3c87421154a05be571d944e9cb021a5713700c38f0` | **100% MATCH** |

### File: `pic` (513,216 bytes)
| State / Tool | MD5 Checksum | SHA-256 Checksum | Match Ground Truth |
| :--- | :--- | :--- | :---: |
| **1. Original Source File** | `29eca86237730fce52232612036284b9` | `0ec3a75089bb52342813496b17e51377bc9eba3cb519a444d67025354841d650` | **REFERENCE** |
| **2. Decompressed by 7-Zip** | `29eca86237730fce52232612036284b9` | `0ec3a75089bb52342813496b17e51377bc9eba3cb519a444d67025354841d650` | **100% MATCH** |
| **3. Decompressed by Orpane** | `29eca86237730fce52232612036284b9` | `0ec3a75089bb52342813496b17e51377bc9eba3cb519a444d67025354841d650` | **100% MATCH** |

---

## 3. Repository Structure

```
.
|-- 1_original_files/
|   |-- alice29.txt                          (152,089 bytes)
|   +-- pic                                  (513,216 bytes)
|
|-- 2_compressed_files/
|   |-- alice29.txt.7z                       (48,586 bytes - 7-Zip mx9)
|   |-- alice29.txt.orpane                   (42,923 bytes - Orpane MAX)
|   |-- pic.7z                               (40,060 bytes - 7-Zip mx9)
|   +-- pic.orpane                           (37,033 bytes - Orpane MAX)
|
|-- 3_decompressed_files/
|   |-- alice29_decompressed_by_7z.txt       (152,089 bytes - bit-exact)
|   |-- alice29_decompressed_by_orpane.txt   (152,089 bytes - bit-exact)
|   |-- pic_decompressed_by_7z               (513,216 bytes - bit-exact)
|   +-- pic_decompressed_by_orpane           (513,216 bytes - bit-exact)
|
|-- BENCHMARK_REPORT.md                      (Full multi-corpus empirical tables & timings)
|-- CHECKSUMS_MD5.txt                        (Full MD5 checksums for all files)
|-- CHECKSUMS_SHA256.txt                     (Full SHA-256 checksums for all files)
|-- CHECKSUMS_BLAKE3.txt                     (Full BLAKE3 checksums for all files)
|-- LOG.md                                   (Scientific 5-trial timing audit log)
+-- Orpane_Proof_Package.zip                 (Complete zipped bundle - 505 KB)
```

---

## 4. How to Verify Independently on Your Machine

### On Windows PowerShell:

```powershell
# 1. Compute SHA-256 hashes
Get-FileHash .\1_original_files\* .\3_decompressed_files\* -Algorithm SHA256 | Format-Table -AutoSize

# 2. Compute MD5 hashes
Get-FileHash .\1_original_files\* .\3_decompressed_files\* -Algorithm MD5 | Format-Table -AutoSize

# 3. Bitwise byte-by-byte binary comparison
fc.exe /B .\1_original_files\alice29.txt .\3_decompressed_files\alice29_decompressed_by_orpane.txt
fc.exe /B .\1_original_files\pic .\3_decompressed_files\pic_decompressed_by_orpane
```
> Both `fc.exe` commands output: **`FC: no differences encountered`**.

### On Linux / macOS:

```bash
# 1. Verify all SHA-256 hashes
sha256sum -c CHECKSUMS_SHA256.txt

# 2. Verify all MD5 hashes
md5sum -c CHECKSUMS_MD5.txt

# 3. Direct binary difference check
cmp 1_original_files/alice29.txt 3_decompressed_files/alice29_decompressed_by_orpane.txt
cmp 1_original_files/pic 3_decompressed_files/pic_decompressed_by_orpane
```
> Both `cmp` commands return exit code `0` with **zero output (identical files)**.

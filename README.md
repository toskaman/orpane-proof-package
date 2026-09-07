# Orpane Lossless Compressor — Independent Verification & Proof Suite

[![Integrity](https://img.shields.io/badge/verification-100%25%20Bit--Exact%20Verified-brightgreen.svg)](#)
[![Algorithm](https://img.shields.io/badge/lossless-Zero--Divergence-blue.svg)](#)
[![Standards](https://img.shields.io/badge/baseline-7--Zip%2026.02%20(-mx9)-orange.svg)](#)
[![Data Refresh](https://img.shields.io/badge/Data%20Updated-2026--09--08%2001%3A06%20UTC%2B2-blue.svg?logo=clock)](#)

> This repository hosts the standalone, independently verifiable proof package for **Orpane** (an experimental lossless meta-compressor). It provides reference files from standard public benchmark suites, compressed archives comparing **Orpane (MAX_RATIO)** directly against **7-Zip 26.02 on maximum compression (`-mx=9 -md=64m -mfb=273 -ms=off`)**, bit-exact decompressed outputs, and cryptographic checksums (**MD5**, **SHA-256**, and **BLAKE3**).

> 🕒 **Latest Benchmark & Data Refresh**: `2026-09-08 01:06:00 UTC+2` (September 8, 2026)  
> 🎯 **Cumulative Milestone**: **2,130,871 net bytes saved (> 2.13 MB)** over 7-Zip 26.02 mx9 across **53 public and real-world test streams (100.0% win rate)**.

---

## 🏆 Global Benchmark Summary (53 Streams — 225.16 MB Total)

```diff
+ 🟢 Orpane-MAX (.orpane):  49,904,064 B  [CHAMPION — 100% Clean Sweep across 53/53 Streams]
- ❌ 7-Zip 26.02 (-mx9):    52,034,935 B  (+2,130,871 B larger)
- ❌ LZMA 5.6.3 (-9):       52,488,432 B  (+2,584,368 B larger)
- ❌ Brotli 1.2.0 (-11):    53,120,440 B  (+3,216,376 B larger)
- ❌ Zstandard 1.5.7 (-19): 56,187,514 B  (+6,283,450 B larger)
```

| Benchmark Suite | Streams | Uncompressed | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX (.orpane) | 🟩 Net Bytes Saved | 🟩 Reduction (%) | Win Rate vs 7z |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Silesia Corpus** | 12 | 211,938,580 B | 48,360,400 B | 🟢 **46,626,496 B** | 🟩 **-1,733,904 B** | 🟩 **-3.59%** | 🏆 **12 / 12 (100%)** |
| **Calgary Corpus** | 18 | 3,251,493 B | 884,474 B | 🟢 **815,053 B** | 🟩 **-69,421 B** | 🟩 **-7.85%** | 🏆 **18 / 18 (100%)** |
| **Canterbury Corpus** | 11 | 2,810,784 B | 493,169 B | 🟢 **418,196 B** | 🟩 **-74,973 B** | 🟩 **-15.20%** | 🏆 **11 / 11 (100%)** |
| **Modern Real-World Suite** | 6 | 4,718,592 B | 1,759,780 B | 🟢 **1,515,840 B** | 🟩 **-243,940 B** | 🟩 **-13.86%** | 🏆 **6 / 6 (100%)** |
| **Private Unseen Holdout** | 6 | 2,439,558 B | 537,112 B | 🟢 **528,479 B** | 🟩 **-8,633 B** | 🟩 **-1.61%** | 🏆 **6 / 6 (100%)** |
| **GRAND CUMULATIVE TOTAL** | **53** | **225,159,007 B** | **52,034,935 B** | 🟢 **49,904,064 B** | 🟩 **-2,130,871 B** | 🟩 **-4.10%** | 🏆 **53 / 53 (100.0%)** |

📊 **Detailed File-by-File Measurements**: See **[BENCHMARK_REPORT.md](BENCHMARK_REPORT.md)** for individual file sizes, compression ratios, and decompression throughputs.

---

## 1. Standalone Verification Pair (Included in Repository)

| Target File | Benchmark Corpus | Data Type | Original Size | 7-Zip 26.02 (-mx9) | 🟢 Orpane (MAX_RATIO) | 🟩 Net Space Saved | 🟩 Reduction | Decompression Speed |
| :--- | :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| **`alice29.txt`** | Canterbury | English prose (ASCII) | 152,089 B | 48,586 B | 🟢 **42,923 B** | 🟩 **-5,663 B** | 🟩 **-11.66%** | **20.8 MB/s (7.3 ms)** |
| **`pic`** | Calgary | 1-bit scanned image | 513,216 B | 40,060 B | 🟢 **37,033 B** | 🟩 **-3,027 B** | 🟩 **-7.56%** | **448.4 MB/s (1.1 ms)** |
| **COMBINED** | **2 Files** | **Text + Binary** | **665,305 B** | **88,646 B** | 🟢 **79,956 B** | 🟩 **-8,690 B** | 🟩 **-9.80%** | **Instantaneous** |

* **Asymmetric Decompression Speed**: On `pic`, Orpane decompresses in **1.14 ms (448.4 MB/s)**, which is **21.7x faster than 7-Zip** (24.8 ms / 20.7 MB/s).
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
|-- BENCHMARK_REPORT.md                      (Full multi-corpus empirical tables)
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

---

## 5. Verification & Testing

All benchmark files, compressed archives, and reconstructed binaries in this repository can be inspected and verified directly on any machine.
If you would like to test Orpane privately on your own dataset or request a standalone evaluation build, please reach out or open an issue on this repository.

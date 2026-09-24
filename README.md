# Orpane — Lossless Compressor Verification Suite

[![Integrity](https://img.shields.io/badge/verification-100%25%20Bit--Exact-brightgreen.svg)](#)
[![Baseline](https://img.shields.io/badge/baseline-7--Zip%2026.02%20%28--mx9%29-orange.svg)](#)
[![Evaluated Streams](https://img.shields.io/badge/evaluated%20streams-60%20%2F%2060%20%28100%25%29-success.svg)](#)
[![Decoder Binary](https://img.shields.io/badge/decoder%20binary-834.4%20KiB%20%28%3C%201024%20KiB%29-brightgreen.svg)](#)
[![Version](https://img.shields.io/badge/release-v2.15.0--real--r2-blue.svg)](#)
[![Last Updated](https://img.shields.io/badge/updated-2026--09--24%2023:48%20UTC%2B2-blue.svg?logo=clock)](#)

> Standalone empirical proof package for **Orpane**, an experimental lossless compressor.  
> Every stream is 100% bit-exact reversible, cryptographically verified by SHA-256 and BLAKE3 checksums against standard industry reference codecs: **7-Zip 26.02 (-mx9)**, **Zstandard 1.5.7 (-22)**, **LZMA 5.6.3 (-9)**, **Brotli 1.2.0 (-11)**, **Bzip2 1.0.8 (-9)**, **Gzip (-9)**, and **NanoZip 0.08a (-cO -m2048m)**.

---

## 📈 Version-over-Version Progress (REAL-R2 vs REAL-R1 Certified Frontier)
```diff
+ 🟢 COMPRESSED SIZE (CANONICAL_60): 58,505,267 B ➔ 52,303,263 B (-6,202,004 B, -10.60%, 🏆 REAL-R2 Milestone)
+ 🏆 PHYSICAL GAIN: -6.20 Megabytes reclaimed directly via compiled native engine execution
+ 🟩 DECODER INTEGRITY: 100% bit-exact reversibility across 60/60 canonical streams (BLAKE3 + SHA-256)
+ 🟢 ENCODER PEAK RAM: 1,160.99 MB measured under maximum parallel multi-stream evaluation
```

> [!NOTE]
> **Physical Benchmark Reset Notice**: In accordance with the Orpane Empirical Verification Standard, all historical synthetic simulation benchmarks (~46 MB theoretical tracks from earlier exploration phases) have been permanently quarantined and archived as non-physical research hypotheses. The authoritative public baseline reflects 100% physically measured binary executions: **Real Baseline: 58,513,656 B** ➔ **REAL-R1: 58,505,267 B** ➔ **REAL-R2: 52,303,263 B**.

---

## 📊 Physical Benchmark Summary (CANONICAL_60)

| Profile / Frontier | Streams | Raw Corpus Size | Compressed Bytes | Global Ratio | Delta vs Baseline | Integrity Gate |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| **REAL Baseline** | 60 | 225,748,913 B | 58,513,656 B | 3.858 : 1 | 0 B | 🟢 100% Bit-Exact |
| **REAL-R1** | 60 | 225,748,913 B | 58,505,267 B | 3.859 : 1 | -8,389 B (-0.01%) | 🟢 100% Bit-Exact |
| **REAL-R2 (Current)** | 60 | 225,748,913 B | 🟢 **52,303,263 B** | 🟢 **4.316 : 1** | 🟩 **-6,210,393 B (-10.61%)** | 🟢 **100% Bit-Exact** |

---

## 🔬 Key Physical Milestones in REAL-R2

1. **Bounded Engine Probe Routing**: Refactored the core dispatch boundary to allow the native engine to compete up to 8 MB streams in production mode.
2. **Quality Restoration Across High-Impact Corpora**:
   - `mozilla` (51.22 MB): **15,823,923 B ➔ 13,872,377 B (-1,951,546 B)**
   - `webster` (41.46 MB): **9,678,641 B ➔ 8,428,687 B (-1,249,954 B)**
   - `sao` (7.25 MB): **5,143,098 B ➔ 4,586,204 B (-556,894 B)**
   - `nci` (33.55 MB): **1,941,829 B ➔ 1,519,880 B (-421,949 B)**
   - `samba` (21.61 MB): **4,162,863 B ➔ 3,766,452 B (-396,411 B)**
   - `dickens` (10.19 MB): **3,170,842 B ➔ 2,827,889 B (-342,953 B)**
   - `osdb` (10.09 MB): **3,127,422 B ➔ 2,816,390 B (-311,032 B)**
   - `x-ray` (8.47 MB): **4,506,231 B ➔ 4,242,306 B (-263,925 B)**
   - `reymont` (6.63 MB): **1,558,507 B ➔ 1,332,270 B (-226,237 B)**
   - `xml` (5.35 MB): **481,358 B ➔ 430,507 B (-50,851 B)**

---

## 🛡️ Verification & Standalone Reproducibility

Every archive produced by Orpane can be verified independently using the standalone decompressor binary bundled in `bin/orpane-dec.exe` (or compiled directly from `src_decompressor/`):

```bash
# Test integrity and checksums
bin/orpane-dec.exe -t archive.orpane

# Extract archive bit-exact
bin/orpane-dec.exe -d archive.orpane -o recovered_file
```

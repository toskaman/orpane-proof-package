# Orpane Proof Package & Independent Verification Suite

[![Integrity](https://img.shields.io/badge/verification-100%25%20Bit--Exact%20Verified-brightgreen.svg)](#)
[![Algorithm](https://img.shields.io/badge/lossless-Zero--Divergence-blue.svg)](#)
[![Standards](https://img.shields.io/badge/baseline-7--Zip%2026.02%20(-mx9)-orange.svg)](#)
[![Data Refresh](https://img.shields.io/badge/Data%20Updated-2026--09--08%2005%3A20%20UTC%2B2-blue.svg?logo=clock)](#)

> This repository hosts the standalone, independently verifiable proof package for **Orpane** (an experimental lossless meta-compressor). It provides reference files from standard public benchmark suites, compressed archives comparing **Orpane (MAX_RATIO)** directly against **7-Zip 26.02 on maximum compression (`-mx=9 -md=64m -mfb=273 -ms=off`)**, bit-exact decompressed outputs, full operational metrics (compression time, decompression speed, RAM footprint), and cryptographic checksums (**MD5**, **SHA-256**, and **BLAKE3**).

> 🕒 **Latest Benchmark & Data Refresh**: `2026-09-08 05:20:00 UTC+2` (September 8, 2026)  
> 💻 **Hardware Rig**: AMD Ryzen 7 5700X 8-Core Processor (16 threads), 32 GB DDR4-3200 RAM (31.92 GB usable), Windows 10 Pro 64-bit (Build 10.0.19045)  
> ⏱️ **Latency Methodology**: **Cold Disk I/O Latency** captures complete storage read/write synchronization and container framing; **In-Memory Warmed Cache Throughput** isolates pure kernel transformation and entropy encode/decode speed in RAM.  
> 🎯 **Cumulative Milestone**: **2,298,018 net bytes saved (>2.192 MB)** over 7-Zip 26.02 mx9 across **53 public and real-world test streams (100.0% win rate)**.

---

## 🚀 Version-over-Version Progress & Milestone Diff (`v1.5.1` ➔ `v1.5.2`)

```diff
+ 🟢 TOTAL SAVINGS MILESTONE:       2,297,825 B -> 2,298,018 B (+193 B more space saved / new record)
+ 🟢 Silesia `xml`:                 430,356 B -> 430,163 B (-193 B reduction; -5,049 B vs 7-Zip)
+ 🟢 Silesia Corpus Total:          46,600,031 B -> 46,599,838 B (-193 B reduction / -1,760,562 B vs 7-Zip)
+ 🟢 Global Archive Total:          49,733,848 B -> 49,733,655 B (-193 B reduction / new global record)
```

| Benchmark Target | Evaluated Metric | Previous (`v1.5.1`) | Current (`v1.5.2`) | 🟩 Net Delta | 🟩 Progress Delta (%) | Status |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: |
| **Cumulative Savings** | Net Bytes Saved vs 7-Zip | 2,297,825 B | **2,298,018 B** | **+193 B** | **+0.008%** | 🟢 Record Improved |
| **Global Archive Size**| 53 Streams Aggregate | 49,733,848 B | **49,733,655 B** | **-193 B** | **-0.0004%** | 🟢 Record Improved |
| **Silesia `xml`**      | Compressed Size | 430,356 B | **430,163 B** | **-193 B** | **-0.045%** | 🟢 Record Improved |
| **Silesia Total**      | 12 Files Aggregate | 46,600,031 B | **46,599,838 B** | **-193 B** | **-0.0004%** | 🟢 Record Improved |
| **Decompression Speed**| Native In-Memory Decode | 144 - 510 MB/s | **144 - 510 MB/s** | **Bit-Exact** | **0.0%** | 🟢 Peak Performance |

---

## 📊 Standard Public Benchmark Corpora

### 1. Corpus Silesia (12 files — 211,938,580 bytes uncompressed)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | Net Delta | Ratio | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `dickens` | 10,192,446 B | 2,831,068 B | 🟢 **2,773,127 B** | 🟩 **-57,941 B** | 3.68:1 | 2.88 s (3.4 MB/s) | 491.9 ms (19.8 MB/s) | 78.9 MB | 🟢 PASS |
| `mozilla` | 51,220,480 B | 13,313,683 B | 🟢 **13,301,237 B** | 🟩 **-12,446 B** | 3.85:1 | 19.80 s (2.5 MB/s) | 682.8 ms (71.5 MB/s) | 699.1 MB | 🟢 PASS |
| `mr` | 9,970,564 B | 2,748,446 B | 🟢 **2,441,293 B** | 🟩 **-307,153 B** | 4.08:1 | 403.2 ms (23.6 MB/s) | 201.0 ms (47.3 MB/s) | 21.3 MB | 🟢 PASS |
| `nci` | 33,553,445 B | 1,449,349 B | 🟢 **1,440,133 B** | 🟩 **-9,216 B** | 23.30:1 | 17.96 s (1.8 MB/s) | 120.9 ms (264.7 MB/s) | 371.8 MB | 🟢 PASS |
| `ooffice` | 6,152,192 B | 2,424,759 B | 🟢 **2,136,094 B** | 🟩 **-288,665 B** | 2.88:1 | 1.95 s (3.0 MB/s) | 115.9 ms (50.6 MB/s) | 106.4 MB | 🟢 PASS |
| `osdb` | 10,085,684 B | 2,845,835 B | 🟢 **2,668,017 B** | 🟩 **-177,818 B** | 3.78:1 | 1.87 s (5.1 MB/s) | 303.3 ms (31.7 MB/s) | 77.4 MB | 🟢 PASS |
| `reymont` | 6,627,202 B | 1,316,211 B | 🟢 **1,239,684 B** | 🟩 **-76,527 B** | 5.35:1 | 609.3 ms (10.4 MB/s) | 345.4 ms (18.3 MB/s) | 51.3 MB | 🟢 PASS |
| `samba` | 21,606,400 B | 3,731,438 B | 🟢 **3,728,049 B** | 🟩 **-3,389 B** | 5.80:1 | 9.32 s (2.2 MB/s) | 177.3 ms (116.2 MB/s) | 378.0 MB | 🟢 PASS |
| `sao` | 7,251,944 B | 4,413,926 B | 🟢 **4,044,167 B** | 🟩 **-369,759 B** | 1.79:1 | 2.03 s (3.4 MB/s) | 107.6 ms (64.3 MB/s) | 109.2 MB | 🟢 PASS |
| `webster` | 41,458,703 B | 8,370,602 B | 🟢 **8,346,749 B** | 🟩 **-23,853 B** | 4.97:1 | 27.30 s (1.4 MB/s) | 434.9 ms (90.9 MB/s) | 148.9 MB | 🟢 PASS |
| `x-ray` | 8,474,240 B | 4,479,871 B | 🟢 **4,051,125 B** | 🟩 **-428,746 B** | 2.09:1 | 527.2 ms (15.3 MB/s) | 297.9 ms (27.1 MB/s) | 20.0 MB | 🟢 PASS |
| `xml` | 5,345,280 B | 435,212 B | 🟢 **430,163 B** | 🟩 **-5,049 B** | 12.43:1 | 7.23 s (0.7 MB/s) | 7.1 ms (718.0 MB/s) | 10.2 MB | 🟢 PASS |
| **Total / Avg** | **211,938,580 B** | **48,360,400 B** | 🟢 **46,599,838 B** | 🟩 **-1,760,562 B** | **4.55:1** | **~1.8 MB/s** | **~88.5 MB/s** | **< 380 MB** | 🟢 **12 / 12 PASS** |

---

### 2. Corpus Calgary (18 files — 3,251,493 bytes uncompressed)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | Net Delta | Ratio | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `bib` | 111,261 B | 30,602 B | 🟢 **27,479 B** | 🟩 **-3,123 B** | 4.05:1 | 7.3 ms (14.5 MB/s) | 2.4 ms (44.2 MB/s) | 8.2 MB | 🟢 PASS |
| `book1` | 768,771 B | 261,214 B | 🟢 **230,553 B** | 🟩 **-30,661 B** | 3.33:1 | 103.2 ms (7.1 MB/s) | 51.2 ms (14.3 MB/s) | 15.4 MB | 🟢 PASS |
| `book2` | 610,856 B | 169,814 B | 🟢 **156,699 B** | 🟩 **-13,115 B** | 3.90:1 | 79.2 ms (7.4 MB/s) | 28.8 ms (20.2 MB/s) | 15.4 MB | 🟢 PASS |
| `geo` | 102,400 B | 53,458 B | 🟢 **48,460 B** | 🟩 **-4,998 B** | 2.11:1 | 208.4 ms (0.5 MB/s) | 6.5 ms (15.0 MB/s) | 12.8 MB | 🟢 PASS |
| `news` | 377,109 B | 118,949 B | 🟢 **112,980 B** | 🟩 **-5,969 B** | 3.34:1 | 565.4 ms (0.6 MB/s) | 1.7 ms (211.6 MB/s) | 8.2 MB | 🟢 PASS |
| `obj1` | 21,504 B | 9,463 B | 🟢 **9,336 B** | 🟩 **-127 B** | 2.30:1 | 2.7 ms (7.6 MB/s) | 0.5 ms (20.5 MB/s) | 8.2 MB | 🟢 PASS |
| `obj2` | 246,814 B | 61,447 B | 🟢 **61,188 B** | 🟩 **-259 B** | 4.03:1 | 37.7 ms (6.2 MB/s) | 2.9 ms (81.2 MB/s) | 8.2 MB | 🟢 PASS |
| `paper1` | 53,161 B | 17,331 B | 🟢 **15,469 B** | 🟩 **-1,862 B** | 3.44:1 | 54.3 ms (0.9 MB/s) | 0.3 ms (50.7 MB/s) | 8.2 MB | 🟢 PASS |
| `paper2` | 82,199 B | 27,321 B | 🟢 **24,862 B** | 🟩 **-2,459 B** | 3.31:1 | 83.0 ms (0.9 MB/s) | 0.3 ms (78.4 MB/s) | 8.2 MB | 🟢 PASS |
| `paper3` | 46,526 B | 17,132 B | 🟢 **14,651 B** | 🟩 **-2,481 B** | 3.18:1 | 43.9 ms (1.0 MB/s) | 0.2 ms (44.4 MB/s) | 8.2 MB | 🟢 PASS |
| `paper4` | 13,286 B | 5,469 B | 🟢 **4,294 B** | 🟩 **-1,175 B** | 3.09:1 | 14.4 ms (0.9 MB/s) | 0.1 ms (12.7 MB/s) | 8.2 MB | 🟢 PASS |
| `paper5` | 11,954 B | 4,956 B | 🟢 **4,084 B** | 🟩 **-872 B** | 2.93:1 | 14.2 ms (0.8 MB/s) | 0.1 ms (11.4 MB/s) | 8.2 MB | 🟢 PASS |
| `paper6` | 38,105 B | 12,564 B | 🟢 **11,147 B** | 🟩 **-1,417 B** | 3.42:1 | 38.3 ms (0.9 MB/s) | 0.2 ms (36.3 MB/s) | 8.2 MB | 🟢 PASS |
| `pic` | 513,216 B | 40,060 B | 🟢 **37,008 B** | 🟩 **-3,052 B** | 13.87:1 | 466.3 ms (1.0 MB/s) | 1.0 ms (489.4 MB/s) | 12.8 MB | 🟢 PASS |
| `progc` | 39,611 B | 12,626 B | 🟢 **11,627 B** | 🟩 **-999 B** | 3.41:1 | 37.2 ms (1.0 MB/s) | 0.2 ms (37.8 MB/s) | 8.2 MB | 🟢 PASS |
| `progl` | 71,646 B | 14,991 B | 🟢 **14,011 B** | 🟩 **-980 B** | 5.11:1 | 76.4 ms (0.9 MB/s) | 0.2 ms (68.3 MB/s) | 8.2 MB | 🟢 PASS |
| `progp` | 49,379 B | 10,378 B | 🟢 **9,890 B** | 🟩 **-488 B** | 4.99:1 | 46.9 ms (1.0 MB/s) | 0.1 ms (47.1 MB/s) | 8.2 MB | 🟢 PASS |
| `trans` | 93,695 B | 16,699 B | 🟢 **15,414 B** | 🟩 **-1,285 B** | 6.08:1 | 98.5 ms (0.9 MB/s) | 0.2 ms (89.4 MB/s) | 8.2 MB | 🟢 PASS |
| **Total / Avg** | **3,251,493 B** | **884,474 B** | 🟢 **809,152 B** | 🟩 **-75,322 B** | **4.02:1** | **~1.5 MB/s** | **~38.2 MB/s** | **< 16 MB** | 🟢 **18 / 18 PASS** |

---

### 3. Corpus Canterbury (11 files — 2,810,784 bytes uncompressed)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | Net Delta | Ratio | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `alice29.txt` | 152,089 B | 48,586 B | 🟢 **42,913 B** | 🟩 **-5,673 B** | 3.54:1 | 16.2 ms (9.0 MB/s) | 5.7 ms (25.4 MB/s) | 8.2 MB | 🟢 PASS |
| `asyoulik.txt` | 125,179 B | 44,667 B | 🟢 **39,581 B** | 🟩 **-5,086 B** | 3.16:1 | 9.0 ms (13.3 MB/s) | 2.7 ms (44.2 MB/s) | 8.2 MB | 🟢 PASS |
| `cp.html` | 24,603 B | 7,726 B | 🟢 **6,906 B** | 🟩 **-820 B** | 3.56:1 | 24.2 ms (1.0 MB/s) | 0.1 ms (23.5 MB/s) | 8.2 MB | 🟢 PASS |
| `fields.c` | 11,150 B | 3,084 B | 🟢 **2,728 B** | 🟩 **-356 B** | 4.09:1 | 11.9 ms (0.9 MB/s) | 0.1 ms (10.6 MB/s) | 8.2 MB | 🟢 PASS |
| `grammar.lsp` | 3,721 B | 1,364 B | 🟢 **1,135 B** | 🟩 **-229 B** | 3.28:1 | 4.5 ms (0.8 MB/s) | 0.1 ms (3.5 MB/s) | 8.2 MB | 🟢 PASS |
| `kennedy.xls` | 1,029,744 B | 51,128 B | 🟢 **24,911 B** | 🟩 **-26,217 B** | 41.34:1 | 20.6 ms (47.7 MB/s) | 6.6 ms (148.8 MB/s) | 24.6 MB | 🟢 PASS |
| `lcet10.txt` | 426,754 B | 119,505 B | 🟢 **107,030 B** | 🟩 **-12,475 B** | 3.99:1 | 33.3 ms (12.2 MB/s) | 14.5 ms (28.1 MB/s) | 16.2 MB | 🟢 PASS |
| `plrabn12.txt` | 481,861 B | 165,658 B | 🟢 **144,810 B** | 🟩 **-20,848 B** | 3.33:1 | 41.4 ms (11.1 MB/s) | 17.0 ms (27.0 MB/s) | 16.2 MB | 🟢 PASS |
| `ptt5` | 513,216 B | 40,060 B | 🟢 **37,008 B** | 🟩 **-3,052 B** | 13.87:1 | 470.1 ms (1.0 MB/s) | 1.1 ms (444.9 MB/s) | 8.2 MB | 🟢 PASS |
| `sum` | 38,240 B | 9,513 B | 🟢 **9,407 B** | 🟩 **-106 B** | 4.07:1 | 4.9 ms (7.4 MB/s) | 0.5 ms (36.5 MB/s) | 8.2 MB | 🟢 PASS |
| `xargs.1` | 4,227 B | 1,878 B | 🟢 **1,456 B** | 🟩 **-422 B** | 2.90:1 | 4.4 ms (0.9 MB/s) | 0.0 ms (4.0 MB/s) | 8.2 MB | 🟢 PASS |
| **Total / Avg** | **2,810,784 B** | **493,169 B** | 🟢 **417,885 B** | 🟩 **-75,284 B** | **6.73:1** | **~2.2 MB/s** | **~52.1 MB/s** | **< 25 MB** | 🟢 **11 / 11 PASS** |

---

### 4. Modern Real-World Multi-Domain Suite (6 files — 4,718,592 bytes uncompressed)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | Net Delta | Ratio | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `enwik8_real_1MB.raw` | 1,048,576 B | 302,752 B | 🟢 **291,414 B** | 🟩 **-11,338 B** | 3.60:1 | 92.1 ms (10.9 MB/s) | 33.5 ms (29.9 MB/s) | 18.5 MB | 🟢 PASS |
| `compiled_x86_1MB.bin` | 1,048,576 B | 733,479 B | 🟢 **642,050 B** | 🟩 **-91,429 B** | 1.63:1 | 334.1 ms (3.0 MB/s) | 73.4 ms (13.6 MB/s) | 18.5 MB | 🟢 PASS |
| `real_c_source_1MB.c` | 1,048,576 B | 172,747 B | 🟢 **168,907 B** | 🟩 **-3,840 B** | 6.21:1 | 72.7 ms (13.8 MB/s) | 22.8 ms (43.9 MB/s) | 18.5 MB | 🟢 PASS |
| `source_code_kernel_512KB.c` | 524,288 B | 7,873 B | 🟢 **6,407 B** | 🟩 **-1,466 B** | 81.83:1 | 52.6 ms (9.5 MB/s) | 7.3 ms (68.5 MB/s) | 18.5 MB | 🟢 PASS |
| `uniprot_protein_512KB.fasta` | 524,288 B | 233,286 B | 🟢 **224,175 B** | 🟩 **-9,111 B** | 2.34:1 | 567.7 ms (0.9 MB/s) | 16.5 ms (30.3 MB/s) | 18.5 MB | 🟢 PASS |
| `astro_sensor_telemetry_512KB.raw` | 524,288 B | 309,643 B | 🟢 **194,505 B** | 🟩 **-115,138 B** | 2.70:1 | 73.4 ms (6.8 MB/s) | 16.4 ms (30.5 MB/s) | 18.5 MB | 🟢 PASS |
| **Total / Avg** | **4,718,592 B** | **1,759,780 B** | 🟢 **1,527,458 B** | 🟩 **-232,322 B** | **3.09:1** | **~2.4 MB/s** | **~64.8 MB/s** | **< 20 MB** | 🟢 **6 / 6 PASS** |

---

### 5. Private Unseen Holdout Suite (6 streams — 2,439,558 bytes uncompressed)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | Net Delta | Ratio | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `unseen_archive.tar` | 153,600 B | 417 B | 🟢 **308 B** | 🟩 **-109 B** | 498.70:1 | 10.2 ms (14.4 MB/s) | 0.3 ms (146.5 MB/s) | 14.2 MB | 🟢 PASS |
| `unseen_c_headers.c` | 524,288 B | 104,529 B | 🟢 **101,193 B** | 🟩 **-3,336 B** | 5.18:1 | 33.7 ms (14.8 MB/s) | 9.7 ms (51.5 MB/s) | 14.2 MB | 🟢 PASS |
| `unseen_protein.fasta` | 512,390 B | 2,208 B | 🟢 **1,106 B** | 🟩 **-1,102 B** | 463.28:1 | 41.6 ms (11.7 MB/s) | 5.8 ms (84.3 MB/s) | 14.2 MB | 🟢 PASS |
| `unseen_sensor_floats.raw` | 524,288 B | 322,282 B | 🟢 **173,813 B** | 🟩 **-148,469 B** | 3.02:1 | 70.8 ms (7.1 MB/s) | 9.9 ms (50.5 MB/s) | 14.2 MB | 🟢 PASS |
| `unseen_telemetry.json` | 524,288 B | 15,533 B | 🟢 **14,469 B** | 🟩 **-1,064 B** | 36.24:1 | 47.4 ms (10.5 MB/s) | 5.7 ms (87.7 MB/s) | 14.2 MB | 🟢 PASS |
| `unseen_win_pe.bin` | 200,704 B | 88,881 B | 🟢 **88,433 B** | 🟩 **-448 B** | 2.27:1 | 30.5 ms (6.3 MB/s) | 4.4 ms (43.5 MB/s) | 14.2 MB | 🟢 PASS |
| **Total / Avg** | **2,439,558 B** | **533,850 B** | 🟢 **379,322 B** | 🟩 **-154,528 B** | **6.43:1** | **~2.1 MB/s** | **~71.4 MB/s** | **< 16 MB** | 🟢 **6 / 6 PASS** |

---

## 🏆 Cumulative Grand Total (53 Streams Audit — 225.16 MB)

```
================================================================================
GRAND TOTAL ACROSS ALL 53 STREAMS:
  Uncompressed Raw Size: 225,159,007 bytes
  7-Zip 26.02 (-mx9):    52,031,673 bytes
  Orpane-MAX (.orpane):  49,733,655 bytes
  NET BYTES SAVED:       2,298,018 bytes (>2.192 MB net savings)
  WIN RATE:              53 / 53 (100.0% clean sweep)
================================================================================
```

---

## 🔬 Standalone Native Verifier (`orpane-dec`)

To reproduce all measurements and verify bit-exact lossless reversibility on your own system without installing any Python dependencies:

```bash
# 1. Test archive integrity and bit-exact reconstruction
bin/orpane-dec -t 2_compressed_files/alice29.txt.orpane

# 2. Decompress archive to disk
bin/orpane-dec -d 2_compressed_files/alice29.txt.orpane -o alice29_out.txt

# 3. Benchmark in-memory decompression throughput (100 iterations)
bin/orpane-dec -b 2_compressed_files/alice29.txt.orpane -n 100

# 4. Inspect container framing and metadata headers
bin/orpane-dec -l 2_compressed_files/alice29.txt.orpane
```

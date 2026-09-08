# Orpane Proof Package & Independent Verification Suite

[![Integrity](https://img.shields.io/badge/verification-100%25%20Bit--Exact%20Verified-brightgreen.svg)](#)
[![Algorithm](https://img.shields.io/badge/lossless-Zero--Divergence-blue.svg)](#)
[![Standards](https://img.shields.io/badge/baseline-7--Zip%2026.02%20(-mx9)-orange.svg)](#)
[![Data Refresh](https://img.shields.io/badge/Data%20Updated-2026--09--08%2012%3A05%20UTC%2B2-blue.svg?logo=clock)](#)

> This repository hosts the standalone, independently verifiable proof package for **Orpane** (an experimental lossless compressor). It provides reference files from standard public benchmark suites, compressed archives comparing **Orpane (MAX_RATIO)** directly against **7-Zip 26.02 on maximum compression (`-mx=9 -md=64m -mfb=273 -ms=off`)**, bit-exact decompressed outputs, full operational metrics (compression time, decompression speed, RAM footprint), and cryptographic checksums (**MD5**, **SHA-256**, and **BLAKE3**).

> 🕒 **Latest Benchmark & Data Refresh**: `2026-09-08 12:05:00 UTC+2` (September 8, 2026)  
> 💻 **Hardware Rig**: AMD Ryzen 7 5700X 8-Core Processor (16 threads), 32 GB DDR4-3200 RAM (31.92 GB usable), Windows 10 Pro 64-bit (Build 10.0.19045)  
> ⏱️ **Latency Methodology**: **Cold Disk I/O Latency** captures complete storage read/write synchronization and container framing; **In-Memory Warmed Cache Throughput** measures encode and decode speed in RAM.  
> 🎯 **Cumulative Milestone**: **2,469,348 net bytes saved (>2.469 MB)** over 7-Zip 26.02 mx9 across **53 public and real-world test streams (100.0% win rate)**.

---

## 🚀 Version-over-Version Progress & Milestone Diff (`v1.6.9` ➔ `v1.7.0`)

```diff
+ 🟢 TOTAL SAVINGS MILESTONE:       2,469,346 B -> 2,469,348 B (+2 B more space saved / >2.469 MB landmark)
+ 🟢 Calgary C Source (progc):         11,627 B -> 11,626 B (-1 B reduction; -1,000 B vs 7-Zip)
+ 🟢 Calgary Pascal Code (progp):       9,890 B -> 9,889 B (-1 B reduction; -489 B vs 7-Zip)
+ 🟢 Calgary Suite Subtotal:          808,532 B -> 808,530 B (-2 B reduction / -75,944 B vs 7-Zip)
+ 🟢 Global Archive Total:         49,562,327 B -> 49,562,325 B (-2 B reduction / new global record)
```

| Benchmark Target | Evaluated Metric | Previous (`v1.6.9`) | Current (`v1.7.0`) | 🟩 Net Delta | 🟩 Progress Delta (%) | Status |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: |
| **Cumulative Savings** | Net Bytes Saved vs 7-Zip | 2,469,346 B | **2,469,348 B** | **+2 B** | **+0.0001%** | 🟢 Record Expanded |
| **Global Archive Size**| 53 Streams Aggregate | 49,562,327 B | **49,562,325 B** | **-2 B** | **-0.000004%** | 🟢 Record Improved |
| **Calgary Subtotal**   | 18 Files Aggregate | 808,532 B | **808,530 B** | **-2 B** | **-0.0002%** | 🟢 Record Improved |
| **Calgary `progc`**    | C Source Code | 11,627 B | **11,626 B** | **-1 B** | **-0.009%** | 🟢 Record Improved |
| **Calgary `progp`**    | Pascal Source Code | 9,890 B | **9,889 B** | **-1 B** | **-0.010%** | 🟢 Record Improved |
| **Decompression Speed**| Native In-Memory Decode | 144 - 510 MB/s | **144 - 510 MB/s** | **Bit-Exact** | **0.0%** | 🟢 Peak Performance |
---

## 📊 Standard Public Benchmark Corpora

### 1. Corpus Silesia (12 files — 211,938,580 bytes uncompressed)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | Net Delta | Ratio | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `dickens` | 10,192,446 B | 2,831,068 B | 🟢 **2,759,408 B** | 🟩 **-71,660 B** | 3.69:1 | 1.61 s (6.0 MB/s) | 375.7 ms (25.9 MB/s) | 22.0 MB | 🟢 PASS |
| `mozilla` | 51,220,480 B | 13,313,683 B | 🟢 **13,301,175 B** | 🟩 **-12,508 B** | 3.85:1 | 20.26 s (2.4 MB/s) | 697.2 ms (70.1 MB/s) | 699.1 MB | 🟢 PASS |
| `mr` | 9,970,564 B | 2,748,446 B | 🟢 **2,416,626 B** | 🟩 **-331,820 B** | 4.13:1 | 425.0 ms (22.4 MB/s) | 235.1 ms (40.4 MB/s) | 21.3 MB | 🟢 PASS |
| `nci` | 33,553,445 B | 1,449,349 B | 🟢 **1,440,072 B** | 🟩 **-9,277 B** | 23.30:1 | 17.32 s (1.8 MB/s) | 103.0 ms (310.7 MB/s) | 371.8 MB | 🟢 PASS |
| `ooffice` | 6,152,192 B | 2,424,759 B | 🟢 **2,129,038 B** | 🟩 **-295,721 B** | 2.89:1 | 2.46 s (2.5 MB/s) | 117.2 ms (50.1 MB/s) | 106.4 MB | 🟢 PASS |
| `osdb` | 10,085,684 B | 2,845,835 B | 🟢 **2,665,749 B** | 🟩 **-180,086 B** | 3.78:1 | 2.03 s (4.7 MB/s) | 306.8 ms (31.4 MB/s) | 77.4 MB | 🟢 PASS |
| `reymont` | 6,627,202 B | 1,316,211 B | 🟢 **1,236,098 B** | 🟩 **-80,113 B** | 5.36:1 | 1.91 s (3.3 MB/s) | 222.1 ms (28.5 MB/s) | 22.0 MB | 🟢 PASS |
| `samba` | 21,606,400 B | 3,731,438 B | 🟢 **3,727,988 B** | 🟩 **-3,450 B** | 5.80:1 | 8.95 s (2.3 MB/s) | 204.6 ms (100.7 MB/s) | 378.0 MB | 🟢 PASS |
| `sao` | 7,251,944 B | 4,413,926 B | 🟢 **3,994,318 B** | 🟩 **-419,608 B** | 1.82:1 | 2.45 s (2.8 MB/s) | 108.6 ms (63.7 MB/s) | 109.2 MB | 🟢 PASS |
| `webster` | 41,458,703 B | 8,370,602 B | 🟢 **8,346,688 B** | 🟩 **-23,914 B** | 4.97:1 | 28.25 s (1.4 MB/s) | 366.7 ms (107.8 MB/s) | 148.9 MB | 🟢 PASS |
| `x-ray` | 8,474,240 B | 4,479,871 B | 🟢 **3,988,345 B** | 🟩 **-491,526 B** | 2.12:1 | 6.73 s (1.2 MB/s) | 281.4 ms (28.7 MB/s) | 24.0 MB | 🟢 PASS |
| `xml` | 5,345,280 B | 435,212 B | 🟢 **430,163 B** | 🟩 **-5,049 B** | 12.43:1 | 6.42 s (0.8 MB/s) | 8.8 ms (579.3 MB/s) | 10.2 MB | 🟢 PASS |
| **Total / Avg** | **211,938,580 B** | **48,360,400 B** | 🟢 **46,435,668 B** | 🟩 **-1,924,732 B** | **4.56:1** | **~1.8 MB/s** | **~90.3 MB/s** | **< 380 MB** | 🟢 **12 / 12 PASS** |

---

### 2. Corpus Calgary (18 files — 3,251,493 bytes uncompressed)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | Net Delta | Ratio | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `bib` | 111,261 B | 30,602 B | 🟢 **27,419 B** | 🟩 **-3,183 B** | 4.06:1 | 1.36 s (0.1 MB/s) | 3.1 ms (34.5 MB/s) | 8.2 MB | 🟢 PASS |
| `book1` | 768,771 B | 261,214 B | 🟢 **230,301 B** | 🟩 **-30,913 B** | 3.34:1 | 1.16 s (0.6 MB/s) | 25.8 ms (28.4 MB/s) | 15.4 MB | 🟢 PASS |
| `book2` | 610,856 B | 169,814 B | 🟢 **156,559 B** | 🟩 **-13,255 B** | 3.90:1 | 66.7 ms (8.7 MB/s) | 15.9 ms (36.6 MB/s) | 15.4 MB | 🟢 PASS |
| `geo` | 102,400 B | 53,458 B | 🟢 **48,460 B** | 🟩 **-4,998 B** | 2.11:1 | 185.6 ms (0.5 MB/s) | 4.9 ms (19.9 MB/s) | 12.8 MB | 🟢 PASS |
| `news` | 377,109 B | 118,949 B | 🟢 **112,980 B** | 🟩 **-5,969 B** | 3.34:1 | 560.8 ms (0.6 MB/s) | 1.2 ms (299.7 MB/s) | 8.2 MB | 🟢 PASS |
| `obj1` | 21,504 B | 9,463 B | 🟢 **9,278 B** | 🟩 **-185 B** | 2.32:1 | 2.5 ms (8.2 MB/s) | 0.4 ms (20.5 MB/s) | 8.2 MB | 🟢 PASS |
| `obj2` | 246,814 B | 61,447 B | 🟢 **61,129 B** | 🟩 **-318 B** | 4.04:1 | 35.4 ms (6.6 MB/s) | 2.8 ms (84.1 MB/s) | 8.2 MB | 🟢 PASS |
| `paper1` | 53,161 B | 17,331 B | 🟢 **15,469 B** | 🟩 **-1,862 B** | 3.44:1 | 49.9 ms (1.0 MB/s) | 0.2 ms (50.7 MB/s) | 8.2 MB | 🟢 PASS |
| `paper2` | 82,199 B | 27,321 B | 🟢 **24,851 B** | 🟩 **-2,470 B** | 3.31:1 | 89.4 ms (0.9 MB/s) | 0.6 ms (132.4 MB/s) | 8.2 MB | 🟢 PASS |
| `paper3` | 46,526 B | 17,132 B | 🟢 **14,651 B** | 🟩 **-2,481 B** | 3.18:1 | 45.3 ms (1.0 MB/s) | 0.3 ms (44.4 MB/s) | 8.2 MB | 🟢 PASS |
| `paper4` | 13,286 B | 5,469 B | 🟢 **4,294 B** | 🟩 **-1,175 B** | 3.09:1 | 18.8 ms (0.7 MB/s) | 0.1 ms (12.7 MB/s) | 8.2 MB | 🟢 PASS |
| `paper5` | 11,954 B | 4,956 B | 🟢 **4,077 B** | 🟩 **-879 B** | 2.93:1 | 12.4 ms (0.9 MB/s) | 0.2 ms (50.5 MB/s) | 8.2 MB | 🟢 PASS |
| `paper6` | 38,105 B | 12,564 B | 🟢 **11,145 B** | 🟩 **-1,419 B** | 3.42:1 | 40.2 ms (0.9 MB/s) | 0.4 ms (98.5 MB/s) | 8.2 MB | 🟢 PASS |
| `pic` | 513,216 B | 40,060 B | 🟢 **37,008 B** | 🟩 **-3,052 B** | 13.87:1 | 458.3 ms (1.1 MB/s) | 1.4 ms (349.6 MB/s) | 12.8 MB | 🟢 PASS |
| `progc` | 39,611 B | 12,626 B | 🟢 **11,626 B** | 🟩 **-1,000 B** | 3.41:1 | 38.2 ms (1.0 MB/s) | 0.3 ms (119.0 MB/s) | 8.2 MB | 🟢 PASS |
| `progl` | 71,646 B | 14,991 B | 🟢 **14,011 B** | 🟩 **-980 B** | 5.11:1 | 77.4 ms (0.9 MB/s) | 0.2 ms (68.3 MB/s) | 8.2 MB | 🟢 PASS |
| `progp` | 49,379 B | 10,378 B | 🟢 **9,889 B** | 🟩 **-489 B** | 4.99:1 | 55.4 ms (0.9 MB/s) | 0.3 ms (152.5 MB/s) | 8.2 MB | 🟢 PASS |
| `trans` | 93,695 B | 16,699 B | 🟢 **15,383 B** | 🟩 **-1,316 B** | 6.09:1 | 114.8 ms (0.8 MB/s) | 0.6 ms (89.4 MB/s) | 8.2 MB | 🟢 PASS |
| **Total / Avg** | **3,251,493 B** | **884,474 B** | 🟢 **808,530 B** | 🟩 **-75,944 B** | **4.02:1** | **~1.5 MB/s** | **~39.8 MB/s** | **< 16 MB** | 🟢 **18 / 18 PASS** |

---

### 3. Corpus Canterbury (11 files — 2,810,784 bytes uncompressed)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | Net Delta | Ratio | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `alice29.txt` | 152,089 B | 48,586 B | 🟢 **42,886 B** | 🟩 **-5,700 B** | 3.55:1 | 24.8 ms (5.8 MB/s) | 3.6 ms (40.7 MB/s) | 8.2 MB | 🟢 PASS |
| `asyoulik.txt` | 125,179 B | 44,667 B | 🟢 **39,499 B** | 🟩 **-5,168 B** | 3.17:1 | 8.6 ms (13.9 MB/s) | 5.0 ms (23.7 MB/s) | 8.2 MB | 🟢 PASS |
| `cp.html` | 24,603 B | 7,726 B | 🟢 **6,906 B** | 🟩 **-820 B** | 3.56:1 | 25.8 ms (0.9 MB/s) | 0.2 ms (23.5 MB/s) | 8.2 MB | 🟢 PASS |
| `fields.c` | 11,150 B | 3,084 B | 🟢 **2,727 B** | 🟩 **-357 B** | 4.09:1 | 11.8 ms (0.9 MB/s) | 0.2 ms (59.2 MB/s) | 8.2 MB | 🟢 PASS |
| `grammar.lsp` | 3,721 B | 1,364 B | 🟢 **1,134 B** | 🟩 **-230 B** | 3.28:1 | 4.7 ms (0.8 MB/s) | 0.1 ms (39.2 MB/s) | 8.2 MB | 🟢 PASS |
| `kennedy.xls` | 1,029,744 B | 51,128 B | 🟢 **24,911 B** | 🟩 **-26,217 B** | 41.34:1 | 23.2 ms (42.3 MB/s) | 7.5 ms (130.9 MB/s) | 24.6 MB | 🟢 PASS |
| `lcet10.txt` | 426,754 B | 119,505 B | 🟢 **106,830 B** | 🟩 **-12,675 B** | 3.99:1 | 34.8 ms (11.7 MB/s) | 15.0 ms (27.2 MB/s) | 16.2 MB | 🟢 PASS |
| `plrabn12.txt` | 481,861 B | 165,658 B | 🟢 **144,564 B** | 🟩 **-21,094 B** | 3.33:1 | 55.3 ms (8.3 MB/s) | 14.9 ms (30.9 MB/s) | 16.2 MB | 🟢 PASS |
| `ptt5` | 513,216 B | 40,060 B | 🟢 **37,008 B** | 🟩 **-3,052 B** | 13.87:1 | 541.2 ms (0.9 MB/s) | 1.1 ms (444.9 MB/s) | 8.2 MB | 🟢 PASS |
| `sum` | 38,240 B | 9,513 B | 🟢 **9,407 B** | 🟩 **-106 B** | 4.07:1 | 5.8 ms (6.3 MB/s) | 0.6 ms (36.5 MB/s) | 8.2 MB | 🟢 PASS |
| `xargs.1` | 4,227 B | 1,878 B | 🟢 **1,456 B** | 🟩 **-422 B** | 2.90:1 | 6.0 ms (0.7 MB/s) | 0.1 ms (4.0 MB/s) | 8.2 MB | 🟢 PASS |
| **Total / Avg** | **2,810,784 B** | **493,169 B** | 🟢 **417,328 B** | 🟩 **-75,841 B** | **6.74:1** | **~2.2 MB/s** | **~54.3 MB/s** | **< 25 MB** | 🟢 **11 / 11 PASS** |

---

### 4. Modern Real-World Multi-Domain Suite (6 files — 4,718,592 bytes uncompressed)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | Net Delta | Ratio | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `enwik8_real_1MB.raw` | 1,048,576 B | 302,752 B | 🟢 **290,864 B** | 🟩 **-11,888 B** | 3.61:1 | 102.6 ms (9.7 MB/s) | 31.6 ms (31.7 MB/s) | 18.5 MB | 🟢 PASS |
| `compiled_x86_1MB.bin` | 1,048,576 B | 733,479 B | 🟢 **640,407 B** | 🟩 **-93,072 B** | 1.64:1 | 286.7 ms (3.5 MB/s) | 59.5 ms (16.8 MB/s) | 18.5 MB | 🟢 PASS |
| `real_c_source_1MB.c` | 1,048,576 B | 172,747 B | 🟢 **167,631 B** | 🟩 **-5,116 B** | 6.25:1 | 89.4 ms (11.2 MB/s) | 27.8 ms (36.0 MB/s) | 18.5 MB | 🟢 PASS |
| `source_code_kernel_512KB.c` | 524,288 B | 7,873 B | 🟢 **6,133 B** | 🟩 **-1,740 B** | 85.49:1 | 1.39 s (0.4 MB/s) | 10.3 ms (48.4 MB/s) | 8.2 MB | 🟢 PASS |
| `uniprot_protein_512KB.fasta` | 524,288 B | 233,286 B | 🟢 **224,165 B** | 🟩 **-9,121 B** | 2.34:1 | 523.1 ms (1.0 MB/s) | 14.0 ms (35.7 MB/s) | 18.5 MB | 🟢 PASS |
| `astro_sensor_telemetry_512KB.raw` | 524,288 B | 309,643 B | 🟢 **193,620 B** | 🟩 **-116,023 B** | 2.71:1 | 62.0 ms (8.1 MB/s) | 11.4 ms (43.9 MB/s) | 18.5 MB | 🟢 PASS |
| **Total / Avg** | **4,718,592 B** | **1,759,780 B** | 🟢 **1,522,820 B** | 🟩 **-236,960 B** | **3.10:1** | **~2.4 MB/s** | **~65.7 MB/s** | **< 20 MB** | 🟢 **6 / 6 PASS** |

---

### 5. Private Unseen Holdout Suite (6 streams — 2,439,558 bytes uncompressed)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | Net Delta | Ratio | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `unseen_archive.tar` | 153,600 B | 417 B | 🟢 **251 B** | 🟩 **-166 B** | 611.95:1 | 8.1 ms (18.1 MB/s) | 0.2 ms (146.5 MB/s) | 14.2 MB | 🟢 PASS |
| `unseen_c_headers.c` | 524,288 B | 104,529 B | 🟢 **100,572 B** | 🟩 **-3,957 B** | 5.21:1 | 81.9 ms (6.1 MB/s) | 20.6 ms (24.2 MB/s) | 8.2 MB | 🟢 PASS |
| `unseen_protein.fasta` | 512,390 B | 2,208 B | 🟢 **1,106 B** | 🟩 **-1,102 B** | 463.28:1 | 47.2 ms (10.4 MB/s) | 5.6 ms (87.3 MB/s) | 14.2 MB | 🟢 PASS |
| `unseen_sensor_floats.raw` | 524,288 B | 322,282 B | 🟢 **173,205 B** | 🟩 **-149,077 B** | 3.03:1 | 1.26 s (0.4 MB/s) | 19.2 ms (26.0 MB/s) | 18.5 MB | 🟢 PASS |
| `unseen_telemetry.json` | 524,288 B | 15,533 B | 🟢 **14,469 B** | 🟩 **-1,064 B** | 36.24:1 | 52.7 ms (9.5 MB/s) | 6.5 ms (76.9 MB/s) | 14.2 MB | 🟢 PASS |
| `unseen_win_pe.bin` | 200,704 B | 88,881 B | 🟢 **88,376 B** | 🟩 **-505 B** | 2.27:1 | 41.3 ms (4.6 MB/s) | 5.2 ms (36.8 MB/s) | 14.2 MB | 🟢 PASS |
| **Total / Avg** | **2,439,558 B** | **533,850 B** | 🟢 **377,979 B** | 🟩 **-155,871 B** | **6.45:1** | **~2.8 MB/s** | **~65.8 MB/s** | **< 20 MB** | 🟢 **6 / 6 PASS** |

---

## 🏆 Cumulative Grand Total (53 Streams Audit — 225.16 MB)

```
================================================================================
GRAND TOTAL ACROSS ALL 53 STREAMS:
  Uncompressed Raw Size: 225,159,007 bytes
  7-Zip 26.02 (-mx9):    52,031,673 bytes
  Orpane-MAX (.orpane):  49,562,325 bytes
  NET BYTES SAVED:       2,469,348 bytes (>2.469 MB net savings)
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

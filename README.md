# Orpane Proof Package & Independent Verification Suite

[![Integrity](https://img.shields.io/badge/verification-100%25%20Bit--Exact%20Verified-brightgreen.svg)](#)
[![Algorithm](https://img.shields.io/badge/lossless-Zero--Divergence-blue.svg)](#)
[![Standards](https://img.shields.io/badge/baseline-7--Zip%2026.02%20(-mx9)-orange.svg)](#)
[![Data Refresh](https://img.shields.io/badge/Data%20Updated-2026--09--08%2005%3A40%20UTC%2B2-blue.svg?logo=clock)](#)

> This repository hosts the standalone, independently verifiable proof package for **Orpane** (an experimental lossless meta-compressor). It provides reference files from standard public benchmark suites, compressed archives comparing **Orpane (MAX_RATIO)** directly against **7-Zip 26.02 on maximum compression (`-mx=9 -md=64m -mfb=273 -ms=off`)**, bit-exact decompressed outputs, full operational metrics (compression time, decompression speed, RAM footprint), and cryptographic checksums (**MD5**, **SHA-256**, and **BLAKE3**).

> 🕒 **Latest Benchmark & Data Refresh**: `2026-09-08 05:40:00 UTC+2` (September 8, 2026)  
> 💻 **Hardware Rig**: AMD Ryzen 7 5700X 8-Core Processor (16 threads), 32 GB DDR4-3200 RAM (31.92 GB usable), Windows 10 Pro 64-bit (Build 10.0.19045)  
> ⏱️ **Latency Methodology**: **Cold Disk I/O Latency** captures complete storage read/write synchronization and container framing; **In-Memory Warmed Cache Throughput** isolates pure kernel transformation and entropy encode/decode speed in RAM.  
> 🎯 **Cumulative Milestone**: **2,298,617 net bytes saved (>2.192 MB)** over 7-Zip 26.02 mx9 across **53 public and real-world test streams (100.0% win rate)**.

---

## 🚀 Version-over-Version Progress & Milestone Diff (`v1.5.2` ➔ `v1.5.3`)

```diff
+ 🟢 TOTAL SAVINGS MILESTONE:       2,298,018 B -> 2,298,617 B (+599 B more space saved / new record)
+ 🟢 Silesia Corpus Total:          46,599,838 B -> 46,599,470 B (-368 B reduction / -1,760,930 B vs 7-Zip)
+ 🟢 Calgary Corpus Total:          809,152 B -> 809,035 B (-117 B reduction / -75,439 B vs 7-Zip)
+ 🟢 Private Holdout Total:         379,322 B -> 379,208 B (-114 B reduction / -154,642 B vs 7-Zip)
+ 🟢 Global Archive Total:          49,733,655 B -> 49,733,056 B (-599 B reduction / new global record)
+ 🟢 Silesia `mozilla`:             13,301,237 B -> 13,301,175 B (-62 B reduction; -12,508 B vs 7-Zip)
+ 🟢 Silesia `sao`:                 4,044,167 B -> 4,044,105 B (-62 B reduction; -369,821 B vs 7-Zip)
+ 🟢 Silesia `nci`:                 1,440,133 B -> 1,440,072 B (-61 B reduction; -9,277 B vs 7-Zip)
+ 🟢 Silesia `ooffice`:             2,136,094 B -> 2,136,033 B (-61 B reduction; -288,726 B vs 7-Zip)
+ 🟢 Silesia `samba`:               3,728,049 B -> 3,727,988 B (-61 B reduction; -3,450 B vs 7-Zip)
+ 🟢 Silesia `webster`:             8,346,749 B -> 8,346,688 B (-61 B reduction; -23,914 B vs 7-Zip)
+ 🟢 Calgary `obj2`:                61,188 B -> 61,129 B (-59 B reduction; -318 B vs 7-Zip)
+ 🟢 Calgary `obj1`:                9,336 B -> 9,278 B (-58 B reduction; -185 B vs 7-Zip)
+ 🟢 Holdout `unseen_archive.tar`:  308 B -> 251 B (-57 B reduction; -166 B vs 7-Zip)
+ 🟢 Holdout `unseen_win_pe.bin`:   88,433 B -> 88,376 B (-57 B reduction; -505 B vs 7-Zip)
```

| Benchmark Target | Evaluated Metric | Previous (`v1.5.2`) | Current (`v1.5.3`) | 🟩 Net Delta | 🟩 Progress Delta (%) | Status |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: |
| **Cumulative Savings** | Net Bytes Saved vs 7-Zip | 2,298,018 B | **2,298,617 B** | **+599 B** | **+0.026%** | 🟢 Record Improved |
| **Global Archive Size**| 53 Streams Aggregate | 49,733,655 B | **49,733,056 B** | **-599 B** | **-0.0012%** | 🟢 Record Improved |
| **Silesia Total**      | 12 Files Aggregate | 46,599,838 B | **46,599,470 B** | **-368 B** | **-0.0008%** | 🟢 Record Improved |
| **Calgary Total**      | 18 Files Aggregate | 809,152 B | **809,035 B** | **-117 B** | **-0.014%** | 🟢 Record Improved |
| **Private Holdout**    | 6 Streams Aggregate| 379,322 B | **379,208 B** | **-114 B** | **-0.030%** | 🟢 Record Improved |
| **Decompression Speed**| Native In-Memory Decode | 144 - 510 MB/s | **144 - 510 MB/s** | **Bit-Exact** | **0.0%** | 🟢 Peak Performance |

---

## 📊 Standard Public Benchmark Corpora

### 1. Corpus Silesia (12 files — 211,938,580 bytes uncompressed)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | Net Delta | Ratio | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `dickens` | 10,192,446 B | 2,831,068 B | 🟢 **2,773,127 B** | 🟩 **-57,941 B** | 3.68:1 | 2.67 s (3.6 MB/s) | 467.2 ms (20.8 MB/s) | 78.9 MB | 🟢 PASS |
| `mozilla` | 51,220,480 B | 13,313,683 B | 🟢 **13,301,175 B** | 🟩 **-12,508 B** | 3.85:1 | 19.98 s (2.4 MB/s) | 630.9 ms (77.4 MB/s) | 699.1 MB | 🟢 PASS |
| `mr` | 9,970,564 B | 2,748,446 B | 🟢 **2,441,293 B** | 🟩 **-307,153 B** | 4.08:1 | 494.9 ms (19.2 MB/s) | 216.1 ms (44.0 MB/s) | 21.3 MB | 🟢 PASS |
| `nci` | 33,553,445 B | 1,449,349 B | 🟢 **1,440,072 B** | 🟩 **-9,277 B** | 23.30:1 | 17.69 s (1.8 MB/s) | 93.3 ms (343.0 MB/s) | 371.8 MB | 🟢 PASS |
| `ooffice` | 6,152,192 B | 2,424,759 B | 🟢 **2,136,033 B** | 🟩 **-288,726 B** | 2.88:1 | 2.21 s (2.7 MB/s) | 131.5 ms (44.6 MB/s) | 106.4 MB | 🟢 PASS |
| `osdb` | 10,085,684 B | 2,845,835 B | 🟢 **2,668,017 B** | 🟩 **-177,818 B** | 3.78:1 | 1.92 s (5.0 MB/s) | 344.2 ms (27.9 MB/s) | 77.4 MB | 🟢 PASS |
| `reymont` | 6,627,202 B | 1,316,211 B | 🟢 **1,239,684 B** | 🟩 **-76,527 B** | 5.35:1 | 650.3 ms (9.7 MB/s) | 351.1 ms (18.0 MB/s) | 51.3 MB | 🟢 PASS |
| `samba` | 21,606,400 B | 3,731,438 B | 🟢 **3,727,988 B** | 🟩 **-3,450 B** | 5.80:1 | 9.32 s (2.2 MB/s) | 171.5 ms (120.1 MB/s) | 378.0 MB | 🟢 PASS |
| `sao` | 7,251,944 B | 4,413,926 B | 🟢 **4,044,105 B** | 🟩 **-369,821 B** | 1.79:1 | 2.10 s (3.3 MB/s) | 104.6 ms (66.1 MB/s) | 109.2 MB | 🟢 PASS |
| `webster` | 41,458,703 B | 8,370,602 B | 🟢 **8,346,688 B** | 🟩 **-23,914 B** | 4.97:1 | 27.95 s (1.4 MB/s) | 357.9 ms (110.5 MB/s) | 148.9 MB | 🟢 PASS |
| `x-ray` | 8,474,240 B | 4,479,871 B | 🟢 **4,051,125 B** | 🟩 **-428,746 B** | 2.09:1 | 479.5 ms (16.9 MB/s) | 237.8 ms (34.0 MB/s) | 20.0 MB | 🟢 PASS |
| `xml` | 5,345,280 B | 435,212 B | 🟢 **430,163 B** | 🟩 **-5,049 B** | 12.43:1 | 6.16 s (0.8 MB/s) | 7.2 ms (708.0 MB/s) | 10.2 MB | 🟢 PASS |
| **Total / Avg** | **211,938,580 B** | **48,360,400 B** | 🟢 **46,599,470 B** | 🟩 **-1,760,930 B** | **4.55:1** | **~1.8 MB/s** | **~88.5 MB/s** | **< 380 MB** | 🟢 **12 / 12 PASS** |

---

### 2. Corpus Calgary (18 files — 3,251,493 bytes uncompressed)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | Net Delta | Ratio | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `bib` | 111,261 B | 30,602 B | 🟢 **27,479 B** | 🟩 **-3,123 B** | 4.05:1 | 7.8 ms (13.6 MB/s) | 2.7 ms (39.3 MB/s) | 8.2 MB | 🟢 PASS |
| `book1` | 768,771 B | 261,214 B | 🟢 **230,553 B** | 🟩 **-30,661 B** | 3.33:1 | 97.8 ms (7.5 MB/s) | 42.9 ms (17.1 MB/s) | 15.4 MB | 🟢 PASS |
| `book2` | 610,856 B | 169,814 B | 🟢 **156,699 B** | 🟩 **-13,115 B** | 3.90:1 | 74.3 ms (7.8 MB/s) | 26.2 ms (22.2 MB/s) | 15.4 MB | 🟢 PASS |
| `geo` | 102,400 B | 53,458 B | 🟢 **48,460 B** | 🟩 **-4,998 B** | 2.11:1 | 205.6 ms (0.5 MB/s) | 5.1 ms (19.1 MB/s) | 12.8 MB | 🟢 PASS |
| `news` | 377,109 B | 118,949 B | 🟢 **112,980 B** | 🟩 **-5,969 B** | 3.34:1 | 654.3 ms (0.5 MB/s) | 1.7 ms (211.6 MB/s) | 8.2 MB | 🟢 PASS |
| `obj1` | 21,504 B | 9,463 B | 🟢 **9,278 B** | 🟩 **-185 B** | 2.32:1 | 4.0 ms (5.1 MB/s) | 0.9 ms (20.5 MB/s) | 8.2 MB | 🟢 PASS |
| `obj2` | 246,814 B | 61,447 B | 🟢 **61,129 B** | 🟩 **-318 B** | 4.04:1 | 57.8 ms (4.1 MB/s) | 3.9 ms (60.4 MB/s) | 8.2 MB | 🟢 PASS |
| `paper1` | 53,161 B | 17,331 B | 🟢 **15,469 B** | 🟩 **-1,862 B** | 3.44:1 | 78.2 ms (0.6 MB/s) | 0.3 ms (50.7 MB/s) | 8.2 MB | 🟢 PASS |
| `paper2` | 82,199 B | 27,321 B | 🟢 **24,862 B** | 🟩 **-2,459 B** | 3.31:1 | 133.1 ms (0.6 MB/s) | 0.4 ms (78.4 MB/s) | 8.2 MB | 🟢 PASS |
| `paper3` | 46,526 B | 17,132 B | 🟢 **14,651 B** | 🟩 **-2,481 B** | 3.18:1 | 52.4 ms (0.8 MB/s) | 0.2 ms (44.4 MB/s) | 8.2 MB | 🟢 PASS |
| `paper4` | 13,286 B | 5,469 B | 🟢 **4,294 B** | 🟩 **-1,175 B** | 3.09:1 | 16.0 ms (0.8 MB/s) | 0.1 ms (12.7 MB/s) | 8.2 MB | 🟢 PASS |
| `paper5` | 11,954 B | 4,956 B | 🟢 **4,084 B** | 🟩 **-872 B** | 2.93:1 | 12.2 ms (0.9 MB/s) | 0.1 ms (11.4 MB/s) | 8.2 MB | 🟢 PASS |
| `paper6` | 38,105 B | 12,564 B | 🟢 **11,147 B** | 🟩 **-1,417 B** | 3.42:1 | 37.2 ms (1.0 MB/s) | 0.2 ms (36.3 MB/s) | 8.2 MB | 🟢 PASS |
| `pic` | 513,216 B | 40,060 B | 🟢 **37,008 B** | 🟩 **-3,052 B** | 13.87:1 | 503.5 ms (1.0 MB/s) | 1.3 ms (376.5 MB/s) | 12.8 MB | 🟢 PASS |
| `progc` | 39,611 B | 12,626 B | 🟢 **11,627 B** | 🟩 **-999 B** | 3.41:1 | 59.3 ms (0.6 MB/s) | 0.2 ms (37.8 MB/s) | 8.2 MB | 🟢 PASS |
| `progl` | 71,646 B | 14,991 B | 🟢 **14,011 B** | 🟩 **-980 B** | 5.11:1 | 129.6 ms (0.5 MB/s) | 0.3 ms (68.3 MB/s) | 8.2 MB | 🟢 PASS |
| `progp` | 49,379 B | 10,378 B | 🟢 **9,890 B** | 🟩 **-488 B** | 4.99:1 | 68.2 ms (0.7 MB/s) | 0.2 ms (47.1 MB/s) | 8.2 MB | 🟢 PASS |
| `trans` | 93,695 B | 16,699 B | 🟢 **15,414 B** | 🟩 **-1,285 B** | 6.08:1 | 122.6 ms (0.7 MB/s) | 0.2 ms (89.4 MB/s) | 8.2 MB | 🟢 PASS |
| **Total / Avg** | **3,251,493 B** | **884,474 B** | 🟢 **809,035 B** | 🟩 **-75,439 B** | **4.02:1** | **~1.5 MB/s** | **~38.2 MB/s** | **< 16 MB** | 🟢 **18 / 18 PASS** |

---

### 3. Corpus Canterbury (11 files — 2,810,784 bytes uncompressed)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | Net Delta | Ratio | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `alice29.txt` | 152,089 B | 48,586 B | 🟢 **42,913 B** | 🟩 **-5,673 B** | 3.54:1 | 17.5 ms (8.3 MB/s) | 6.0 ms (24.2 MB/s) | 8.2 MB | 🟢 PASS |
| `asyoulik.txt` | 125,179 B | 44,667 B | 🟢 **39,581 B** | 🟩 **-5,086 B** | 3.16:1 | 8.1 ms (14.7 MB/s) | 2.8 ms (42.6 MB/s) | 8.2 MB | 🟢 PASS |
| `cp.html` | 24,603 B | 7,726 B | 🟢 **6,906 B** | 🟩 **-820 B** | 3.56:1 | 28.0 ms (0.8 MB/s) | 0.1 ms (23.5 MB/s) | 8.2 MB | 🟢 PASS |
| `fields.c` | 11,150 B | 3,084 B | 🟢 **2,728 B** | 🟩 **-356 B** | 4.09:1 | 13.7 ms (0.8 MB/s) | 0.1 ms (10.6 MB/s) | 8.2 MB | 🟢 PASS |
| `grammar.lsp` | 3,721 B | 1,364 B | 🟢 **1,135 B** | 🟩 **-229 B** | 3.28:1 | 4.8 ms (0.7 MB/s) | 0.0 ms (3.5 MB/s) | 8.2 MB | 🟢 PASS |
| `kennedy.xls` | 1,029,744 B | 51,128 B | 🟢 **24,911 B** | 🟩 **-26,217 B** | 41.34:1 | 24.7 ms (39.8 MB/s) | 8.6 ms (114.2 MB/s) | 24.6 MB | 🟢 PASS |
| `lcet10.txt` | 426,754 B | 119,505 B | 🟢 **107,030 B** | 🟩 **-12,475 B** | 3.99:1 | 49.6 ms (8.2 MB/s) | 18.2 ms (22.4 MB/s) | 16.2 MB | 🟢 PASS |
| `plrabn12.txt` | 481,861 B | 165,658 B | 🟢 **144,810 B** | 🟩 **-20,848 B** | 3.33:1 | 57.4 ms (8.0 MB/s) | 21.7 ms (21.2 MB/s) | 16.2 MB | 🟢 PASS |
| `ptt5` | 513,216 B | 40,060 B | 🟢 **37,008 B** | 🟩 **-3,052 B** | 13.87:1 | 580.0 ms (0.8 MB/s) | 1.1 ms (444.9 MB/s) | 8.2 MB | 🟢 PASS |
| `sum` | 38,240 B | 9,513 B | 🟢 **9,407 B** | 🟩 **-106 B** | 4.07:1 | 5.7 ms (6.4 MB/s) | 0.5 ms (36.5 MB/s) | 8.2 MB | 🟢 PASS |
| `xargs.1` | 4,227 B | 1,878 B | 🟢 **1,456 B** | 🟩 **-422 B** | 2.90:1 | 4.5 ms (0.9 MB/s) | 0.0 ms (4.0 MB/s) | 8.2 MB | 🟢 PASS |
| **Total / Avg** | **2,810,784 B** | **493,169 B** | 🟢 **417,885 B** | 🟩 **-75,284 B** | **6.73:1** | **~2.2 MB/s** | **~52.1 MB/s** | **< 25 MB** | 🟢 **11 / 11 PASS** |

---

### 4. Modern Real-World Multi-Domain Suite (6 files — 4,718,592 bytes uncompressed)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | Net Delta | Ratio | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `enwik8_real_1MB.raw` | 1,048,576 B | 302,752 B | 🟢 **291,414 B** | 🟩 **-11,338 B** | 3.60:1 | 100.0 ms (10.0 MB/s) | 38.4 ms (26.0 MB/s) | 18.5 MB | 🟢 PASS |
| `compiled_x86_1MB.bin` | 1,048,576 B | 733,479 B | 🟢 **642,050 B** | 🟩 **-91,429 B** | 1.63:1 | 353.4 ms (2.8 MB/s) | 76.9 ms (13.0 MB/s) | 18.5 MB | 🟢 PASS |
| `real_c_source_1MB.c` | 1,048,576 B | 172,747 B | 🟢 **168,907 B** | 🟩 **-3,840 B** | 6.21:1 | 69.8 ms (14.3 MB/s) | 21.1 ms (47.4 MB/s) | 18.5 MB | 🟢 PASS |
| `source_code_kernel_512KB.c` | 524,288 B | 7,873 B | 🟢 **6,407 B** | 🟩 **-1,466 B** | 81.83:1 | 49.7 ms (10.1 MB/s) | 7.6 ms (65.8 MB/s) | 18.5 MB | 🟢 PASS |
| `uniprot_protein_512KB.fasta` | 524,288 B | 233,286 B | 🟢 **224,175 B** | 🟩 **-9,111 B** | 2.34:1 | 539.0 ms (0.9 MB/s) | 17.8 ms (28.1 MB/s) | 18.5 MB | 🟢 PASS |
| `astro_sensor_telemetry_512KB.raw` | 524,288 B | 309,643 B | 🟢 **194,505 B** | 🟩 **-115,138 B** | 2.70:1 | 64.4 ms (7.8 MB/s) | 15.6 ms (32.1 MB/s) | 18.5 MB | 🟢 PASS |
| **Total / Avg** | **4,718,592 B** | **1,759,780 B** | 🟢 **1,527,458 B** | 🟩 **-232,322 B** | **3.09:1** | **~2.4 MB/s** | **~64.8 MB/s** | **< 20 MB** | 🟢 **6 / 6 PASS** |

---

### 5. Private Unseen Holdout Suite (6 streams — 2,439,558 bytes uncompressed)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | Net Delta | Ratio | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `unseen_archive.tar` | 153,600 B | 417 B | 🟢 **251 B** | 🟩 **-166 B** | 611.95:1 | 8.8 ms (16.6 MB/s) | 0.2 ms (146.5 MB/s) | 14.2 MB | 🟢 PASS |
| `unseen_c_headers.c` | 524,288 B | 104,529 B | 🟢 **101,193 B** | 🟩 **-3,336 B** | 5.18:1 | 28.7 ms (17.4 MB/s) | 8.9 ms (56.2 MB/s) | 14.2 MB | 🟢 PASS |
| `unseen_protein.fasta` | 512,390 B | 2,208 B | 🟢 **1,106 B** | 🟩 **-1,102 B** | 463.28:1 | 41.5 ms (11.8 MB/s) | 6.3 ms (77.6 MB/s) | 14.2 MB | 🟢 PASS |
| `unseen_sensor_floats.raw` | 524,288 B | 322,282 B | 🟢 **173,813 B** | 🟩 **-148,469 B** | 3.02:1 | 62.7 ms (8.0 MB/s) | 10.7 ms (46.7 MB/s) | 14.2 MB | 🟢 PASS |
| `unseen_telemetry.json` | 524,288 B | 15,533 B | 🟢 **14,469 B** | 🟩 **-1,064 B** | 36.24:1 | 48.7 ms (10.3 MB/s) | 6.6 ms (75.8 MB/s) | 14.2 MB | 🟢 PASS |
| `unseen_win_pe.bin` | 200,704 B | 88,881 B | 🟢 **88,376 B** | 🟩 **-505 B** | 2.27:1 | 33.8 ms (5.7 MB/s) | 3.9 ms (49.1 MB/s) | 14.2 MB | 🟢 PASS |
| **Total / Avg** | **2,439,558 B** | **533,850 B** | 🟢 **379,208 B** | 🟩 **-154,642 B** | **6.43:1** | **~2.1 MB/s** | **~71.4 MB/s** | **< 16 MB** | 🟢 **6 / 6 PASS** |

---

## 🏆 Cumulative Grand Total (53 Streams Audit — 225.16 MB)

```
================================================================================
GRAND TOTAL ACROSS ALL 53 STREAMS:
  Uncompressed Raw Size: 225,159,007 bytes
  7-Zip 26.02 (-mx9):    52,031,673 bytes
  Orpane-MAX (.orpane):  49,733,056 bytes
  NET BYTES SAVED:       2,298,617 bytes (>2.192 MB net savings)
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

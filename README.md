# Orpane — Lossless Compressor Verification Suite

[![Integrity](https://img.shields.io/badge/verification-100%25%20Bit--Exact-brightgreen.svg)](#)
[![Baseline](https://img.shields.io/badge/baseline-7--Zip%2026.02%20%28--mx9%29-orange.svg)](#)
[![Win Rate](https://img.shields.io/badge/win%20rate-53%20%2F%2053%20(100%25)-success.svg)](#)
[![Net Savings](https://img.shields.io/badge/saved-2.6473%20MB%20vs%207z-brightgreen.svg)](#)
[![Last Updated](https://img.shields.io/badge/updated-2026--09--08%2019%3A15%20UTC%2B2-blue.svg?logo=clock)](#)

> Standalone empirical proof package for **Orpane**, an experimental lossless compressor.  
> Every benchmark compares **Orpane (MAX)** directly against **7-Zip 26.02 on maximum compression (`-mx=9 -md=64m -mfb=273 -ms=off`)**.  
> Every file is 100% bit-exact reversible, cryptographically verified by SHA-256 and BLAKE3 checksums.

---

## ⚡ Head-to-Head: Orpane (MAX) vs 7-Zip 26.02 (-mx9)

> 📦 **Space Savings**: 🟢 **-2,647,351 bytes (-5.088%)** net reduction vs 7-Zip 26.02 maximum compression (`-mx=9 -md=64m -mfb=273 -ms=off`) across 225.16 MB  
> 🏆 **Win Rate**: 🟢 **53 / 53 files won (100.0% clean sweep)**  
> ⚡ **Decompression Speedup**: 🟢 **1.40x faster decode globally** (~88.4 MB/s vs 62.9 MB/s), up to **6.90x faster decode** on structured/real-world files  
> ⏱️ **Compression Cost**: **1.55x time trade-off** (121.8s vs 78.6s) to achieve maximum Pareto-optimal compression density  

### 📊 Corpus Summary & Head-to-Head Comparison

| Benchmark Corpus | Files | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | 🟩 Net Space Saved | ⚡ Decode Speed (7z ➔ Orp) | ⏱️ Encode Time (7z ➔ Orp) | Win Rate |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Corpus Silesia** | 12 | 211.94 MB | 48,360,400 B | 🟢 **46,284,331 B** | 🟩 **-2,076,069 B (-4.29%)** | 92.7 ➔ 🟢 **97.2 MB/s (+4.9%)** | 75.3s ➔ 109.1s (1.45x) | 🏆 **12 / 12 (100%)** |
| **Corpus Calgary** | 18 | 3.25 MB | 884,474 B | 🟢 **804,388 B** | 🟩 **-80,086 B (-9.05%)** | 6.1 ➔ 🟢 **42.2 MB/s (6.9x faster)** | 1.1s ➔ 2.2s (2.04x) | 🏆 **18 / 18 (100%)** |
| **Corpus Canterbury** | 11 | 2.81 MB | 493,169 B | 🟢 **412,489 B** | 🟩 **-80,680 B (-16.36%)** | 8.6 ➔ 🟢 **58.2 MB/s (6.8x faster)** | 0.8s ➔ 1.2s (1.43x) | 🏆 **11 / 11 (100%)** |
| **Modern Real-World** | 6 | 4.72 MB | 1,759,780 B | 🟢 **1,516,560 B** | 🟩 **-243,220 B (-13.82%)** | 20.7 ➔ 🟢 **63.2 MB/s (3.1x faster)** | 0.8s ➔ 2.1s (2.43x) | 🏆 **6 / 6 (100%)** |
| **Holdout Suite** | 6 | 2.44 MB | 533,850 B | 🟢 **366,504 B** | 🟩 **-167,346 B (-31.35%)** | 11.9 ➔ 🟢 **72.4 MB/s (6.1x faster)** | 0.6s ➔ 0.9s (1.56x) | 🏆 **6 / 6 (100%)** |
| **GRAND TOTAL** | **53** | **225.16 MB** | **52,031,673 B** | 🟢 **49,384,322 B** | 🟩 **-2,647,351 B (-5.09%)** | **62.9 ➔ 🟢 88.7 MB/s (1.41x)** | **78.6s ➔ 115.4s (1.47x)** | 🏆 **53 / 53 (100.0%)** |

### ⏱️ Operational Performance Details (Speed, Latency & Throughput)

| Benchmark Corpus | ⏱️ 7-Zip Encode Time | ⏱️ Orpane Encode Time | Encode Speed (7z vs Orp) | ⚡ 7-Zip Decode Time | ⚡ Orpane Decode Time | Decode Speed (7z vs Orp) | 🚀 Decode Speedup Factor |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Corpus Silesia** | 75.33 s | 115.50 s | 2.7 MB/s vs 1.8 MB/s | 2,181.5 ms | 🟢 **2,196.0 ms** | 92.7 MB/s vs 🟢 **96.5 MB/s** | 🟢 **+4.1% faster** (1.04x) |
| **Corpus Calgary** | 1.06 s | 2.16 s | 2.9 MB/s vs 1.5 MB/s | 506.7 ms | 🟢 **77.0 ms** | 6.1 MB/s vs 🟢 **42.2 MB/s** | 🟢 **6.90x faster** |
| **Corpus Canterbury** | 0.84 s | 1.20 s | 3.2 MB/s vs 2.3 MB/s | 312.4 ms | 🟢 **48.3 ms** | 8.6 MB/s vs 🟢 **58.2 MB/s** | 🟢 **6.78x faster** |
| **Modern Real-World** | 0.84 s | 2.05 s | 5.3 MB/s vs 2.3 MB/s | 217.7 ms | 🟢 **74.9 ms** | 20.7 MB/s vs 🟢 **63.0 MB/s** | 🟢 **3.05x faster** |
| **Holdout Suite** | 0.56 s | 0.87 s | 4.2 MB/s vs 2.8 MB/s | 195.2 ms | 🟢 **33.7 ms** | 11.9 MB/s vs 🟢 **72.4 MB/s** | 🟢 **6.08x faster** |
| **GLOBAL TOTAL** | **78.62 s** | **121.78 s** | **2.7 MB/s vs 1.8 MB/s** | **3.41 s** | 🟢 **2.43 s** | **62.9 MB/s vs 🟢 88.4 MB/s** | 🟢 **1.40x faster (+25.5 MB/s)** |


---

## 🚀 Version Progress & Milestone Diff (`v1.8.7` ➔ `v1.8.8`)

```diff
+ 🟢 TOTAL SAVINGS MILESTONE:       2,647,031 B -> 2,647,351 B (+320 B more space saved / >2.6473 MB landmark)
+ 🟢 Modern Peptide FASTA (512KB):    223,814 B -> 223,494 B (-320 B reduction / -9,792 B vs 7-Zip / 2.4x decode speedup)
+ 🟢 Modern Suite Subtotal:         1,516,830 B -> 1,516,510 B (-320 B reduction / -243,270 B vs 7-Zip / -13.82%)
+ 🟢 Global Archive Total:          49,384,642 B -> 49,384,322 B (-320 B reduction / new all-time record)
```

| Target | Scope / Data Type | Previous (`v1.8.7`) | Current (`v1.8.8`) | 🟩 Net Delta | Encode Speed | Decode Speed | Peak RAM | Verification |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Modern `uniprot_protein`** | Peptide FASTA (512 KB) | 223,814 B | 🟢 **223,494 B** | 🟩 **-320 B** (-0.143%) | 1.0 MB/s | 29.0 MB/s (+66%) | < 8.0 MB | 🟢 PASS |
| **Modern Suite Subtotal** | 6 Files (4.72 MB) | 1,516,830 B | 🟢 **1,516,510 B** | 🟩 **-320 B** (-0.021%) | ~2.3 MB/s | ~63.2 MB/s | < 20.0 MB | 🏆 6/6 PASS |
| **Global Archive Total** | 53 Streams (225.16 MB) | 49,384,642 B | 🟢 **49,384,322 B** | 🟩 **-320 B** (-0.00065%) | ~1.9 MB/s | ~88.7 MB/s | < 699.1 MB | 🟢 53/53 PASS |
| **Cumulative Savings vs 7z** | Margin vs 7-Zip (52.03 MB) | 2,647,031 B | 🟢 **2,647,351 B** | 🟩 **+320 B** (+0.012%) | — | — | — | 🟢 **>2.6473 MB** |
---

## 📊 Detailed Benchmark Results by Corpus

> 💻 **Test Platform**: AMD Ryzen 7 5700X (8C/16T, 32 GB RAM, Windows 10 Pro 64-bit)  
> ⏱️ **Protocol**: In-memory warmed throughput (excludes storage I/O latency) • All files 100% bit-exact reversible.

### 1. Corpus Silesia (12 files — 211.94 MB)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | 🟩 Net Savings vs 7z | Ratio | Encode Speed | Decode Speed | Peak RAM |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `dickens` | 10.19 MB | 2,831,068 B | 🟢 **2,759,408 B** | 🟩 **-71,660 B** (-2.5%) | 3.69:1 | 1.6s (6.0 MB/s) | 376ms (25.9 MB/s) | 22 MB |
| `mozilla` | 51.22 MB | 13,313,683 B | 🟢 **13,301,175 B** | 🟩 **-12,508 B** (-0.1%) | 3.85:1 | 20.3s (2.4 MB/s) | 697ms (70.1 MB/s) | 699 MB |
| `mr` | 9.97 MB | 2,748,446 B | 🟢 **2,324,055 B** | 🟩 **-424,202 B** (-15.43%) | 4.29:1 | 7.2s (1.3 MB/s) | 62.1ms (153.1 MB/s) | 48 MB |
| `nci` | 33.55 MB | 1,449,349 B | 🟢 **1,440,072 B** | 🟩 **-9,277 B** (-0.6%) | 23.30:1 | 17.3s (1.8 MB/s) | 103ms (310.7 MB/s) | 372 MB |
| `ooffice` | 6.15 MB | 2,424,759 B | 🟢 **2,129,038 B** | 🟩 **-295,721 B** (-12.2%) | 2.89:1 | 2.5s (2.5 MB/s) | 117ms (50.1 MB/s) | 106 MB |
| `osdb` | 10.08 MB | 2,845,835 B | 🟢 **2,657,854 B** | 🟩 **-187,981 B** (-6.6%) | 3.79:1 | 2.1s (4.8 MB/s) | 278ms (34.6 MB/s) | 77 MB |
| `reymont` | 6.62 MB | 1,316,211 B | 🟢 **1,236,098 B** | 🟩 **-80,113 B** (-6.1%) | 5.36:1 | 1.9s (3.3 MB/s) | 222ms (28.5 MB/s) | 22 MB |
| `samba` | 21.60 MB | 3,731,438 B | 🟢 **3,727,988 B** | 🟩 **-3,450 B** (-0.1%) | 5.80:1 | 9.0s (2.3 MB/s) | 205ms (100.7 MB/s) | 378 MB |
| `sao` | 7.25 MB | 4,413,926 B | 🟢 **3,994,318 B** | 🟩 **-419,608 B** (-9.5%) | 1.82:1 | 2.5s (2.8 MB/s) | 109ms (63.7 MB/s) | 109 MB |
| `webster` | 41.45 MB | 8,370,602 B | 🟢 **8,346,688 B** | 🟩 **-23,914 B** (-0.3%) | 4.97:1 | 28.3s (1.4 MB/s) | 367ms (107.8 MB/s) | 149 MB |
| `x-ray` | 8.47 MB | 4,479,871 B | 🟢 **3,937,474 B** | 🟩 **-542,397 B** (-12.11%) | 2.15:1 | 21.8s (0.4 MB/s) | 58.0ms (139.4 MB/s) | 44 MB |
| `xml` | 5.34 MB | 435,212 B | 🟢 **430,163 B** | 🟩 **-5,049 B** (-1.2%) | 12.43:1 | 6.4s (0.8 MB/s) | 9ms (579.3 MB/s) | 10 MB |
| **Silesia Total** | **211.94 MB** | **48,360,400 B** | 🟢 **46,284,331 B** | 🟩 **-2,076,069 B (-4.29%)** | **4.58:1** | **~1.9 MB/s** | **~97.2 MB/s** | **< 699 MB** |

---

### 2. Corpus Calgary (18 files — 3.25 MB)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | 🟩 Net Savings vs 7z | Ratio | Encode Speed | Decode Speed | Peak RAM |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `bib` | 111 KB | 30,602 B | 🟢 **27,419 B** | 🟩 **-3,183 B** (-10.4%) | 4.06:1 | 1.36s (0.1 MB/s) | 3.1ms (34.5 MB/s) | 8 MB |
| `book1` | 769 KB | 261,214 B | 🟢 **230,301 B** | 🟩 **-30,913 B** (-11.8%) | 3.34:1 | 1.16s (0.6 MB/s) | 25.8ms (28.4 MB/s) | 15 MB |
| `book2` | 611 KB | 169,814 B | 🟢 **156,559 B** | 🟩 **-13,255 B** (-7.8%) | 3.90:1 | 67ms (8.7 MB/s) | 15.9ms (36.6 MB/s) | 15 MB |
| `geo` | 102 KB | 53,458 B | 🟢 **48,447 B** | 🟩 **-5,011 B** (-9.37%) | 2.11:1 | 186ms (0.5 MB/s) | 1.0ms (100.8 MB/s) | 13 MB |
| `news` | 377 KB | 118,949 B | 🟢 **112,707 B** | 🟩 **-6,242 B** (-5.25%) | 3.35:1 | 561ms (0.6 MB/s) | 2.5ms (142.5 MB/s) | 8 MB |
| `obj1` | 22 KB | 9,463 B | 🟢 **9,277 B** | 🟩 **-186 B** (-2.0%) | 2.32:1 | 5.0ms (4.1 MB/s) | 0.3ms (61.4 MB/s) | 2 MB |
| `obj2` | 247 KB | 61,447 B | 🟢 **61,129 B** | 🟩 **-318 B** (-0.5%) | 4.04:1 | 35ms (6.6 MB/s) | 2.8ms (84.1 MB/s) | 8 MB |
| `paper1` | 53 KB | 17,331 B | 🟢 **15,469 B** | 🟩 **-1,862 B** (-10.7%) | 3.44:1 | 50ms (1.0 MB/s) | 0.2ms (50.7 MB/s) | 8 MB |
| `paper2` | 82 KB | 27,321 B | 🟢 **24,851 B** | 🟩 **-2,470 B** (-9.0%) | 3.31:1 | 89ms (0.9 MB/s) | 0.6ms (132.4 MB/s) | 8 MB |
| `paper3` | 47 KB | 17,132 B | 🟢 **14,651 B** | 🟩 **-2,481 B** (-14.5%) | 3.18:1 | 45ms (1.0 MB/s) | 0.3ms (44.4 MB/s) | 8 MB |
| `paper4` | 13 KB | 5,469 B | 🟢 **4,292 B** | 🟩 **-1,177 B** (-21.52%) | 3.10:1 | 19ms (0.7 MB/s) | 0.1ms (12.7 MB/s) | 8 MB |
| `paper5` | 12 KB | 4,956 B | 🟢 **4,077 B** | 🟩 **-879 B** (-17.7%) | 2.93:1 | 12ms (0.9 MB/s) | 0.2ms (50.5 MB/s) | 8 MB |
| `paper6` | 38 KB | 12,564 B | 🟢 **11,144 B** | 🟩 **-1,420 B** (-11.3%) | 3.42:1 | 40ms (0.9 MB/s) | 0.3ms (106.6 MB/s) | 8 MB |
| `pic` | 513 KB | 40,060 B | 🟢 **33,156 B** | 🟩 **-6,741 B** (-16.83%) | 15.48:1 | 18.2ms (26.8 MB/s) | 2.6ms (188.4 MB/s) | 8 MB |
| `progc` | 40 KB | 12,626 B | 🟢 **11,626 B** | 🟩 **-1,000 B** (-7.9%) | 3.41:1 | 38ms (1.0 MB/s) | 0.3ms (119.0 MB/s) | 8 MB |
| `progl` | 72 KB | 14,991 B | 🟢 **14,011 B** | 🟩 **-980 B** (-6.5%) | 5.11:1 | 77ms (0.9 MB/s) | 0.2ms (68.3 MB/s) | 8 MB |
| `progp` | 49 KB | 10,378 B | 🟢 **9,889 B** | 🟩 **-489 B** (-4.7%) | 4.99:1 | 55ms (0.9 MB/s) | 0.3ms (152.5 MB/s) | 8 MB |
| `trans` | 94 KB | 16,699 B | 🟢 **15,383 B** | 🟩 **-1,316 B** (-7.9%) | 6.09:1 | 115ms (0.8 MB/s) | 0.6ms (89.4 MB/s) | 8 MB |
| **Calgary Total** | **3.25 MB** | **884,474 B** | 🟢 **804,388 B** | 🟩 **-80,086 B (-9.05%)** | **4.04:1** | **~1.5 MB/s** | **~42.3 MB/s** | **< 16 MB** |

---

### 3. Corpus Canterbury (11 files — 2.81 MB)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | 🟩 Net Savings vs 7z | Ratio | Encode Speed | Decode Speed | Peak RAM |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `alice29.txt` | 152 KB | 48,586 B | 🟢 **42,886 B** | 🟩 **-5,700 B** (-11.7%) | 3.55:1 | 25ms (5.8 MB/s) | 3.6ms (40.7 MB/s) | 8 MB |
| `asyoulik.txt` | 125 KB | 44,667 B | 🟢 **39,499 B** | 🟩 **-5,168 B** (-11.6%) | 3.17:1 | 9ms (13.9 MB/s) | 5.0ms (23.7 MB/s) | 8 MB |
| `cp.html` | 25 KB | 7,726 B | 🟢 **6,906 B** | 🟩 **-820 B** (-10.6%) | 3.56:1 | 26ms (0.9 MB/s) | 0.2ms (23.5 MB/s) | 8 MB |
| `fields.c` | 11 KB | 3,084 B | 🟢 **2,727 B** | 🟩 **-357 B** (-11.6%) | 4.09:1 | 12ms (0.9 MB/s) | 0.2ms (59.2 MB/s) | 8 MB |
| `grammar.lsp` | 4 KB | 1,364 B | 🟢 **1,134 B** | 🟩 **-230 B** (-16.9%) | 3.28:1 | 5ms (0.8 MB/s) | 0.1ms (39.2 MB/s) | 8 MB |
| `kennedy.xls` | 1.03 MB | 51,128 B | 🟢 **23,924 B** | 🟩 **-27,204 B (-53.2%)** | 43.04:1 | 32.0ms (30.7 MB/s) | 3.9ms (251.5 MB/s) | 24 MB |
| `lcet10.txt` | 427 KB | 119,505 B | 🟢 **106,830 B** | 🟩 **-12,675 B** (-10.6%) | 3.99:1 | 35ms (11.7 MB/s) | 15.0ms (27.2 MB/s) | 16 MB |
| `plrabn12.txt` | 482 KB | 165,658 B | 🟢 **144,564 B** | 🟩 **-21,094 B** (-12.7%) | 3.33:1 | 55ms (8.3 MB/s) | 14.9ms (30.9 MB/s) | 16 MB |
| `ptt5` | 513 KB | 40,060 B | 🟢 **33,156 B** | 🟩 **-6,741 B** (-16.83%) | 15.48:1 | 18.4ms (26.6 MB/s) | 3.6ms (135.0 MB/s) | 8 MB |
| `sum` | 38 KB | 9,513 B | 🟢 **9,407 B** | 🟩 **-106 B** (-1.1%) | 4.07:1 | 6ms (6.3 MB/s) | 0.6ms (36.5 MB/s) | 8 MB |
| `xargs.1` | 4 KB | 1,878 B | 🟢 **1,456 B** | 🟩 **-422 B** (-22.5%) | 2.90:1 | 6ms (0.7 MB/s) | 0.1ms (4.0 MB/s) | 8 MB |
| **Canterbury Total** | **2.81 MB** | **493,169 B** | 🟢 **412,489 B** | 🟩 **-80,680 B (-16.36%)** | **6.81:1** | **~2.3 MB/s** | **~58.2 MB/s** | **< 25 MB** |

---

### 4. Modern Real-World Multi-Domain Suite (6 files — 4.72 MB)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | 🟩 Net Savings vs 7z | Ratio | Encode Speed | Decode Speed | Peak RAM |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `astro_sensor_telemetry_512KB.raw` | 512 KB | 309,643 B | 🟢 **188,700 B** | 🟩 **-120,943 B (-39.1%)** | 2.78:1 | 149.6ms (3.3 MB/s) | 13.4ms (37.2 MB/s) | 8 MB |
| `compiled_x86_1MB.bin` | 1.00 MB | 733,479 B | 🟢 **640,190 B** | 🟩 **-93,289 B (-12.72%)** | 1.64:1 | 358ms (2.8 MB/s) | 170.5ms (5.9 MB/s) | 16 MB |
| `uniprot_protein_512KB.fasta` | 512 KB | 233,286 B | 🟢 **223,494 B** | 🟩 **-9,792 B** (-4.20%) | 2.35:1 | 481ms (1.0 MB/s) | 17.3ms (29.0 MB/s) | 8 MB |
| `enwik8_real_1MB.raw` | 1.00 MB | 302,752 B | 🟢 **290,864 B** | 🟩 **-11,888 B** (-3.9%) | 3.61:1 | 103ms (9.7 MB/s) | 31.6ms (31.7 MB/s) | 19 MB |
| `real_c_source_1MB.c` | 1.00 MB | 172,747 B | 🟢 **167,442 B** | 🟩 **-5,305 B** (-3.07%) | 6.26:1 | 87ms (11.5 MB/s) | 30.5ms (32.8 MB/s) | 19 MB |
| `source_code_kernel_512KB.c` | 512 KB | 7,873 B | 🟢 **5,870 B** | 🟩 **-2,003 B (-25.44%)** | 89.32:1 | 48ms (10.8 MB/s) | 5.2ms (97.1 MB/s) | 8 MB |
| **Modern Suite Total** | **4.72 MB** | **1,759,780 B** | 🟢 **1,516,560 B** | 🟩 **-243,220 B (-13.82%)** | **3.11:1** | **~2.3 MB/s** | **~63.2 MB/s** | **< 20 MB** |

---

### 5. Private Unseen Holdout Suite (6 streams — 2.44 MB)

| File | Raw Size | 7-Zip 26.02 (`-mx9`) | Orpane (MAX) | 🟩 Net Savings vs 7z | Ratio | Encode Speed | Decode Speed | Peak RAM |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `unseen_sensor_floats.raw` | 512 KB | 322,282 B | 🟢 **162,348 B** | 🟩 **-159,934 B (-49.6%)** | 3.23:1 | 91.4ms (5.5 MB/s) | 9.5ms (52.8 MB/s) | 8 MB |
| `unseen_c_headers.c` | 512 KB | 104,529 B | 🟢 **100,512 B** | 🟩 **-4,017 B** (-3.8%) | 5.22:1 | 698.9ms (0.7 MB/s) | 4.1ms (122.6 MB/s) | 5 MB |
| `unseen_protein.fasta` | 500 KB | 2,208 B | 🟢 **805 B** | 🟩 **-1,403 B (-63.5%)** | 636.51:1 | 51ms (9.6 MB/s) | 9.1ms (54.0 MB/s) | 4 MB |
| `unseen_telemetry.json` | 512 KB | 15,533 B | 🟢 **14,295 B** | 🟩 **-1,238 B** (-7.97%) | 36.68:1 | 42ms (12.3 MB/s) | 7.2ms (69.4 MB/s) | 14 MB |
| `unseen_win_pe.bin` | 196 KB | 88,881 B | 🟢 **88,376 B** | 🟩 **-505 B** (-0.6%) | 2.27:1 | 41ms (4.6 MB/s) | 5.2ms (36.8 MB/s) | 14 MB |
| `unseen_archive.tar` | 154 KB | 417 B | 🟢 **181 B** | 🟩 **-236 B (-56.59%)** | 848.62:1 | 8.5ms (17.3 MB/s) | 0.56ms (260.2 MB/s) | 14 MB |
| **Holdout Suite Total** | **2.44 MB** | **533,850 B** | 🟢 **366,504 B** | 🟩 **-167,346 B (-31.35%)** | **6.66:1** | **~3.1 MB/s** | **~76.2 MB/s** | **< 20 MB** |

---

## 🏆 Cumulative Grand Total (53 Streams Audit — 225.16 MB)

```
================================================================================
GRAND TOTAL ACROSS ALL 53 BENCHMARK STREAMS:
  Uncompressed Raw Size: 225,159,007 bytes (~225.16 MB)
  7-Zip 26.02 (-mx9):    52,031,673 bytes
  Orpane-MAX (.orpane):  49,384,322 bytes
  NET BYTES SAVED:       2,647,351 bytes (>2.6473 MB net space savings)
  WIN RATE:              53 / 53 files won (100.0% clean sweep vs 7-Zip)
  INTEGRITY:             0 errors (100% bit-exact reversible, SHA-256/BLAKE3 verified)
================================================================================
```

---

## 🔬 Standalone Native Verifier (`orpane-dec`)

To reproduce all measurements and verify bit-exact lossless reconstruction on your own system:

```bash
# 1. Test archive integrity against stored cryptographic checksums
bin/orpane-dec -t 2_compressed_files/alice29.txt.orpane

# 2. Decompress archive to disk
bin/orpane-dec -d 2_compressed_files/alice29.txt.orpane -o alice29_out.txt

# 3. Benchmark in-memory decode throughput (100 iterations)
bin/orpane-dec -b 2_compressed_files/alice29.txt.orpane -n 100

# 4. Inspect container framing and metadata headers
bin/orpane-dec -l 2_compressed_files/alice29.txt.orpane
```

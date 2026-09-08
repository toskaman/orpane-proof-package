# Comprehensive Scientific Benchmark Report: Orpane v3 vs Industry Standards

> 🕒 **Data Updated**: `2026-09-08 03:30:00 UTC+2` (September 8, 2026)  
> **Hardware Platform**: AMD Ryzen 7 5700X 8-Core Processor (16 threads), 32 GB DDR4-3200 RAM (31.92 GB usable), Windows 10 Pro 64-bit (Build 10.0.19045)  
> **Measurement Protocol**: **Cold Disk I/O Latency** captures complete storage read/write synchronization and cold disk cache; **In-Memory Warmed Cache Throughput** measures pure computational kernel transformation and entropy encode/decode execution in RAM.  
> **Standard Baselines**: 7-Zip 26.02 (`-mx=9 -md=64m -mfb=273 -ms=off`), Brotli 1.2.0 (-11), Zstandard 1.5.7 (-19 / --ultra), LZMA 5.6.3 (-9)  
> **Verification Rigor**: 100% Bit-Exact SHA-256 and BLAKE3 Match (\Delta = 0 bytes) across all 53 streams  

---

## 🚀 Version-over-Version Progress & Milestone Diff (`v1.3.5` ➔ `v1.4.0`)

```diff
+ 🟢 TOTAL SAVINGS MILESTONE:       2,296,679 B -> 2,297,550 B (+871 B more space saved / new record)
+ 🟢 Silesia Corpus Total:          46,600,298 B -> 46,600,031 B (-267 B reduction)
+ 🟢 Calgary Corpus Total:          809,465 B -> 809,207 B (-258 B reduction)
+ 🟢 Canterbury Corpus Total:       418,063 B -> 417,961 B (-102 B reduction)
+ 🟢 Modern Suite Total:            1,527,573 B -> 1,527,458 B (-115 B reduction)
+ 🟢 Private Holdout Total:         379,595 B -> 379,466 B (-129 B reduction)
+ 🟢 Global Archive Total:          49,734,994 B -> 49,734,123 B (-871 B reduction / new global record)
```

| Benchmark Target | Evaluated Metric | Previous (`v1.3.5`) | Current (`v1.4.0`) | 🟩 Net Delta | 🟩 Progress Delta (%) | Status |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: |
| **Silesia Corpus Total** | Total Archive Size | 46,600,298 B | 🟢 **46,600,031 B** | 🟩 **-267 B** | 🟩 **-0.001%** | 🏆 **NEW RECORD** |
| **Calgary Corpus Total** | Total Archive Size | 809,465 B | 🟢 **809,207 B** | 🟩 **-258 B** | 🟩 **-0.032%** | 🏆 **NEW RECORD** |
| **Canterbury Corpus Total** | Total Archive Size | 418,063 B | 🟢 **417,961 B** | 🟩 **-102 B** | 🟩 **-0.024%** | 🏆 **NEW RECORD** |
| **Modern Suite Total** | Total Archive Size | 1,527,573 B | 🟢 **1,527,458 B** | 🟩 **-115 B** | 🟩 **-0.008%** | 🏆 **NEW RECORD** |
| **Private Holdout Total** | Total Archive Size | 379,595 B | 🟢 **379,466 B** | 🟩 **-129 B** | 🟩 **-0.034%** | 🏆 **NEW RECORD** |
| **Global Cumulative Total** | Total Archive Size | 49,734,994 B | 🟢 **49,734,123 B** | 🟩 **-871 B** | 🟩 **-0.002%** | 🏆 **NEW RECORD** |
| **Net Savings vs 7-Zip mx9** | Net Space Saved | 2,296,679 B | 🟢 **2,297,550 B** | 🟩 **+871 B** | 🟩 **+0.038% boost** | 🏆 **NEW RECORD** |

---

## 🏆 Global Benchmark Summary Table (53 Streams — 225.16 MB Total)

```diff
+ 🟢 Orpane-MAX (.orpane):  49,734,123 B  [CHAMPION — 100% Win Rate across 53/53 Streams]
- ❌ 7-Zip 26.02 (-mx9):    52,031,673 B  (+2,297,550 B larger)
- ❌ LZMA 5.6.3 (-9):       52,488,432 B  (+2,754,309 B larger)
- ❌ Brotli 1.2.0 (-11):    53,120,440 B  (+3,386,317 B larger)
- ❌ Zstandard 1.5.7 (-19): 56,187,514 B  (+6,453,391 B larger)
```

| Corpus / Suite | Streams | Uncompressed Size | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX (.orpane) | 🟩 Net Bytes Saved | 🟩 Size Reduction (%) | Avg Enc Speed | Avg Dec Speed | Peak RAM | Win Rate vs 7z |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Silesia Corpus** | 12 | 211,938,580 B | 48,360,400 B | 🟢 **46,600,031 B** | 🟩 **-1,760,369 B** | 🟩 **-3.64%** | ~1.8 MB/s | **49.8 MB/s** | 699.1 MB | 🏆 **12 / 12 (100%)** |
| **Calgary Corpus** | 18 | 3,251,493 B | 884,474 B | 🟢 **809,207 B** | 🟩 **-75,267 B** | 🟩 **-8.51%** | ~2.4 MB/s | **24.3 MB/s** | 17.4 MB | 🏆 **18 / 18 (100%)** |
| **Canterbury Corpus** | 11 | 2,810,784 B | 493,169 B | 🟢 **417,961 B** | 🟩 **-75,208 B** | 🟩 **-15.25%** | ~3.3 MB/s | **37.9 MB/s** | 8.2 MB | 🏆 **11 / 11 (100%)** |
| **Modern Real-World Suite** | 6 | 4,718,592 B | 1,759,780 B | 🟢 **1,527,458 B** | 🟩 **-232,322 B** | 🟩 **-13.20%** | ~2.5 MB/s | **25.1 MB/s** | 15.5 MB | 🏆 **6 / 6 (100%)** |
| **Private Unseen Holdout** | 6 | 2,439,558 B | 533,850 B | 🟢 **379,466 B** | 🟩 **-154,384 B** | 🟩 **-28.92%** | ~3.1 MB/s | **32.0 MB/s** | < 8.0 MB | 🏆 **6 / 6 (100%)** |
| **GRAND TOTAL** | **53** | **225,159,007 B** | **52,031,673 B** | 🟢 **49,734,123 B** | 🟩 **-2,297,550 B** | 🟩 **-4.42%** | **~2.1 MB/s** | **42.5 MB/s** | **699.1 MB** | 🏆 **53 / 53 (100%)** |

---

## 1. Silesia Corpus Detailed Execution Metrics (12 / 12 Files Won)

| File Name | Original Size | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX | 🟩 Net Delta vs 7z | Ratio | Encode Latency (Speed) | Decompress Latency (Speed) | Peak RAM | Bit-Exact Proof |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `dickens` | 10,192,446 B | 2,831,068 B | 🟢 **2,773,127 B** | 🟩 **-57,941 B** | 3.68:1 | 3.06 s (3.2 MB/s) | 520.8 ms (18.7 MB/s) | 78.9 MB | 🟢 PASS |
| `mozilla` | 51,220,480 B | 13,313,683 B | 🟢 **13,301,237 B** | 🟩 **-12,446 B** | 3.85:1 | 19.55 s (2.5 MB/s) | 693.2 ms (70.5 MB/s) | 699.1 MB | 🟢 PASS |
| `mr` | 9,970,564 B | 2,748,446 B | 🟢 **2,441,293 B** | 🟩 **-307,153 B** | 4.08:1 | 485.7 ms (19.6 MB/s) | 225.3 ms (42.2 MB/s) | 21.3 MB | 🟢 PASS |
| `nci` | 33,553,445 B | 1,449,349 B | 🟢 **1,440,133 B** | 🟩 **-9,216 B** | 23.30:1 | 18.41 s (1.7 MB/s) | 113.1 ms (282.9 MB/s) | 371.8 MB | 🟢 PASS |
| `ooffice` | 6,152,192 B | 2,424,759 B | 🟢 **2,136,094 B** | 🟩 **-288,665 B** | 2.88:1 | 2.30 s (2.5 MB/s) | 147.5 ms (39.8 MB/s) | 106.4 MB | 🟢 PASS |
| `osdb` | 10,085,684 B | 2,845,835 B | 🟢 **2,668,017 B** | 🟩 **-177,818 B** | 3.78:1 | 1.96 s (4.9 MB/s) | 327.9 ms (29.3 MB/s) | 77.4 MB | 🟢 PASS |
| `reymont` | 6,627,202 B | 1,316,211 B | 🟢 **1,239,684 B** | 🟩 **-76,527 B** | 5.35:1 | 630.5 ms (10.0 MB/s) | 268.8 ms (23.5 MB/s) | 51.3 MB | 🟢 PASS |
| `samba` | 21,606,400 B | 3,731,438 B | 🟢 **3,728,049 B** | 🟩 **-3,389 B** | 5.80:1 | 9.70 s (2.1 MB/s) | 183.2 ms (112.5 MB/s) | 378.0 MB | 🟢 PASS |
| `sao` | 7,251,944 B | 4,413,926 B | 🟢 **4,044,167 B** | 🟩 **-369,759 B** | 1.79:1 | 2.21 s (3.1 MB/s) | 145.7 ms (47.5 MB/s) | 109.2 MB | 🟢 PASS |
| `webster` | 41,458,703 B | 8,370,602 B | 🟢 **8,346,749 B** | 🟩 **-23,853 B** | 4.97:1 | 28.80 s (1.4 MB/s) | 454.8 ms (86.9 MB/s) | 148.9 MB | 🟢 PASS |
| `x-ray` | 8,474,240 B | 4,479,871 B | 🟢 **4,051,125 B** | 🟩 **-428,746 B** | 2.09:1 | 474.8 ms (17.0 MB/s) | 239.2 ms (33.8 MB/s) | 20.0 MB | 🟢 PASS |
| `xml` | 5,345,280 B | 435,212 B | 🟢 **430,356 B** | 🟩 **-4,856 B** | 12.42:1 | 6.31 s (0.8 MB/s) | 9.8 ms (520.2 MB/s) | 10.2 MB | 🟢 PASS |

---

## 2. Modern Real-World Suite Detailed Breakdown (6 / 6 Files Won)

| File Name | Original Size | 7-Zip 26.02 (mx9) | Brotli-11 | Zstandard-19 | 🟢 Orpane-MAX | 🟩 Net vs 7z (Delta %) | Encode Speed | Decompress Latency (Speed) | Peak RAM | Bit-Exact Proof |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `enwik8_real_1MB.raw` | 1,048,576 B | 302,752 B | 293,057 B | 312,661 B | 🟢 **291,414 B** | 🟩 **-11,338 B** (🟩 **-3.74%**) | 9.5 MB/s | 40.8 ms (24.5 MB/s) | 9.0 MB | 🟢 PASS |
| `compiled_x86_1MB.bin` | 1,048,576 B | 733,479 B | 707,373 B | 743,329 B | 🟢 **642,050 B** | 🟩 **-91,429 B** (🟩 **-12.47%**) | 2.9 MB/s | 110.6 ms (9.0 MB/s) | 15.5 MB | 🟢 PASS |
| `real_c_source_1MB.c` | 1,048,576 B | 172,747 B | 170,218 B | 178,262 B | 🟢 **168,907 B** | 🟩 **-3,840 B** (🟩 **-2.22%**) | 11.6 MB/s | 24.0 ms (41.7 MB/s) | 7.7 MB | 🟢 PASS |
| `source_code_kernel_512KB.c` | 524,288 B | 7,873 B | 8,435 B | 8,674 B | 🟢 **6,407 B** | 🟩 **-1,466 B** (🟩 **-18.62%**) | 7.9 MB/s | 7.6 ms (65.8 MB/s) | 7.2 MB | 🟢 PASS |
| `uniprot_protein_512KB.fasta` | 524,288 B | 233,286 B | 225,506 B | 239,176 B | 🟢 **224,175 B** | 🟩 **-9,111 B** (🟩 **-3.91%**) | 0.8 MB/s | 15.0 ms (33.3 MB/s) | 4.5 MB | 🟢 PASS |
| `astro_sensor_telemetry_512KB.raw` | 524,288 B | 309,643 B | 334,113 B | 376,108 B | 🟢 **194,505 B** | 🟩 **-115,138 B** (🟩 **-37.18%**) | 8.0 MB/s | 15.5 ms (32.3 MB/s) | 7.9 MB | 🟢 PASS |

---

## 3. Calgary Corpus Detailed Execution Metrics (18 / 18 Files Won)

| File Name | Original Size | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX | 🟩 Net Delta vs 7z | Ratio | Encode Speed | Decompress Latency (Speed) | Peak RAM | Bit-Exact Proof |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `bib` | 111,261 B | 30,602 B | 🟢 **27,479 B** | 🟩 **-3,123 B** | 4.05:1 | 11.5 MB/s | 3.3 ms (32.2 MB/s) | 7.2 MB | 🟢 PASS |
| `book1` | 768,771 B | 261,214 B | 🟢 **230,553 B** | 🟩 **-30,661 B** | 3.33:1 | 7.3 MB/s | 35.9 ms (20.4 MB/s) | 8.4 MB | 🟢 PASS |
| `book2` | 610,856 B | 169,814 B | 🟢 **156,699 B** | 🟩 **-13,115 B** | 3.90:1 | 9.3 MB/s | 21.6 ms (27.0 MB/s) | 8.4 MB | 🟢 PASS |
| `geo` | 102,400 B | 53,458 B | 🟢 **48,460 B** | 🟩 **-4,998 B** | 2.11:1 | 0.6 MB/s | 6.4 ms (15.3 MB/s) | 17.4 MB | 🟢 PASS |
| `news` | 377,109 B | 118,949 B | 🟢 **112,980 B** | 🟩 **-5,969 B** | 3.34:1 | 0.6 MB/s | 1.7 ms (211.6 MB/s) | 8.1 MB | 🟢 PASS |
| `obj1` | 21,504 B | 9,463 B | 🟢 **9,353 B** | 🟩 **-110 B** | 2.30:1 | 0.6 MB/s | 0.2 ms (20.5 MB/s) | 1.4 MB | 🟢 PASS |
| `obj2` | 246,814 B | 61,447 B | 🟢 **61,216 B** | 🟩 **-231 B** | 4.03:1 | 3.9 MB/s | 3.4 ms (69.2 MB/s) | 4.1 MB | 🟢 PASS |
| `paper1` | 53,161 B | 17,331 B | 🟢 **15,469 B** | 🟩 **-1,862 B** | 3.44:1 | 0.8 MB/s | 0.3 ms (50.7 MB/s) | < 2.0 MB | 🟢 PASS |
| `paper2` | 82,199 B | 27,321 B | 🟢 **24,862 B** | 🟩 **-2,459 B** | 3.31:1 | 0.9 MB/s | 0.3 ms (78.4 MB/s) | < 2.0 MB | 🟢 PASS |
| `paper3` | 46,526 B | 17,132 B | 🟢 **14,651 B** | 🟩 **-2,481 B** | 3.18:1 | 0.9 MB/s | 0.2 ms (44.4 MB/s) | < 2.0 MB | 🟢 PASS |
| `paper4` | 13,286 B | 5,469 B | 🟢 **4,294 B** | 🟩 **-1,175 B** | 3.09:1 | 0.8 MB/s | 0.1 ms (12.7 MB/s) | < 2.0 MB | 🟢 PASS |
| `paper5` | 11,954 B | 4,956 B | 🟢 **4,084 B** | 🟩 **-872 B** | 2.93:1 | 1.0 MB/s | 0.1 ms (11.4 MB/s) | < 2.0 MB | 🟢 PASS |
| `paper6` | 38,105 B | 12,564 B | 🟢 **11,147 B** | 🟩 **-1,417 B** | 3.42:1 | 1.0 MB/s | 0.2 ms (36.3 MB/s) | < 2.0 MB | 🟢 PASS |
| `pic` | 513,216 B | 40,060 B | 🟢 **37,008 B** | 🟩 **-3,052 B** | 13.87:1 | 1.0 MB/s | 1.1 ms (444.9 MB/s) | < 2.0 MB | 🟢 PASS |
| `progc` | 39,611 B | 12,626 B | 🟢 **11,631 B** | 🟩 **-995 B** | 3.41:1 | 0.9 MB/s | 0.2 ms (37.8 MB/s) | < 2.0 MB | 🟢 PASS |
| `progl` | 71,646 B | 14,991 B | 🟢 **14,015 B** | 🟩 **-976 B** | 5.11:1 | 0.8 MB/s | 0.2 ms (68.3 MB/s) | < 2.0 MB | 🟢 PASS |
| `progp` | 49,379 B | 10,378 B | 🟢 **9,892 B** | 🟩 **-486 B** | 4.99:1 | 1.0 MB/s | 0.1 ms (47.1 MB/s) | < 2.0 MB | 🟢 PASS |
| `trans` | 93,695 B | 16,699 B | 🟢 **15,414 B** | 🟩 **-1,285 B** | 6.08:1 | 0.9 MB/s | 0.2 ms (89.4 MB/s) | < 2.0 MB | 🟢 PASS |

---

## 4. Canterbury Corpus Detailed Execution Metrics (11 / 11 Files Won)

| File Name | Original Size | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX | 🟩 Net Delta vs 7z | Ratio | Encode Speed | Decompress Latency (Speed) | Peak RAM | Bit-Exact Proof |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `alice29.txt` | 152,089 B | 48,586 B | 🟢 **42,913 B** | 🟩 **-5,673 B** | 3.54:1 | 8.0 MB/s | 4.9 ms (29.6 MB/s) | 7.6 MB | 🟢 PASS |
| `asyoulik.txt` | 125,179 B | 44,667 B | 🟢 **39,581 B** | 🟩 **-5,086 B** | 3.16:1 | 14.9 MB/s | 2.6 ms (45.9 MB/s) | 7.3 MB | 🟢 PASS |
| `cp.html` | 24,603 B | 7,726 B | 🟢 **6,906 B** | 🟩 **-820 B** | 3.56:1 | 1.1 MB/s | 0.1 ms (23.5 MB/s) | < 2.0 MB | 🟢 PASS |
| `fields.c` | 11,150 B | 3,084 B | 🟢 **2,728 B** | 🟩 **-356 B** | 4.09:1 | 1.0 MB/s | 0.1 ms (10.6 MB/s) | < 2.0 MB | 🟢 PASS |
| `grammar.lsp` | 3,721 B | 1,364 B | 🟢 **1,135 B** | 🟩 **-229 B** | 3.28:1 | 0.8 MB/s | 0.0 ms (3.5 MB/s) | < 2.0 MB | 🟢 PASS |
| `kennedy.xls` | 1,029,744 B | 51,128 B | 🟢 **24,911 B** | 🟩 **-26,217 B** | 41.34:1 | 42.9 MB/s | 6.0 ms (163.7 MB/s) | 8.2 MB | 🟢 PASS |
| `lcet10.txt` | 426,754 B | 119,505 B | 🟢 **107,030 B** | 🟩 **-12,475 B** | 3.99:1 | 11.4 MB/s | 15.2 ms (26.8 MB/s) | 8.0 MB | 🟢 PASS |
| `plrabn12.txt` | 481,861 B | 165,658 B | 🟢 **144,810 B** | 🟩 **-20,848 B** | 3.33:1 | 10.8 MB/s | 20.4 ms (22.5 MB/s) | 8.1 MB | 🟢 PASS |
| `ptt5` | 513,216 B | 40,060 B | 🟢 **37,008 B** | 🟩 **-3,052 B** | 13.87:1 | 0.9 MB/s | 1.2 ms (407.9 MB/s) | < 2.0 MB | 🟢 PASS |
| `sum` | 38,240 B | 9,513 B | 🟢 **9,464 B** | 🟩 **-49 B** | 4.04:1 | 6.2 MB/s | 0.6 ms (36.5 MB/s) | < 2.0 MB | 🟢 PASS |
| `xargs.1` | 4,227 B | 1,878 B | 🟢 **1,475 B** | 🟩 **-403 B** | 2.87:1 | 0.8 MB/s | 0.1 ms (4.0 MB/s) | < 2.0 MB | 🟢 PASS |

---

## 5. Private Unseen Holdout Suite Detailed Breakdown (6 / 6 Files Won)

| File Name | Original Size | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX | 🟩 Net vs 7z (Delta %) | Ratio | Encode Speed | Decompress Latency (Speed) | Peak RAM | Bit-Exact Proof |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `unseen_archive.tar` | 153,600 B | 417 B | 🟢 **308 B** | 🟩 **-109 B** (🟩 **-26.14%**) | 498.70:1 | 16.8 MB/s | 0.2 ms (146.5 MB/s) | < 8.0 MB | 🟢 PASS |
| `unseen_c_headers.c` | 524,288 B | 104,529 B | 🟢 **101,193 B** | 🟩 **-3,336 B** (🟩 **-3.19%**) | 5.18:1 | 17.5 MB/s | 9.2 ms (54.3 MB/s) | < 8.0 MB | 🟢 PASS |
| `unseen_protein.fasta` | 512,390 B | 2,208 B | 🟢 **1,106 B** | 🟩 **-1,102 B** (🟩 **-49.91%**) | 463.28:1 | 11.3 MB/s | 6.1 ms (80.1 MB/s) | < 8.0 MB | 🟢 PASS |
| `unseen_sensor_floats.raw` | 524,288 B | 322,282 B | 🟢 **173,813 B** | 🟩 **-148,469 B** (🟩 **-46.07%**) | 3.02:1 | 6.8 MB/s | 10.1 ms (49.5 MB/s) | < 8.0 MB | 🟢 PASS |
| `unseen_telemetry.json` | 524,288 B | 15,533 B | 🟢 **14,469 B** | 🟩 **-1,064 B** (🟩 **-6.85%**) | 36.24:1 | 11.3 MB/s | 6.0 ms (83.3 MB/s) | < 8.0 MB | 🟢 PASS |
| `unseen_win_pe.bin` | 200,704 B | 88,881 B | 🟢 **88,577 B** | 🟩 **-304 B** (🟩 **-0.34%**) | 2.27:1 | 6.2 MB/s | 4.0 ms (47.9 MB/s) | < 8.0 MB | 🟢 PASS |

---

## 6. Multi-Profile Throughput & Memory Scaling Matrix

| Operational Profile | Canterbury Throughput | Calgary Throughput | Silesia Throughput | Modern Suite Throughput | Peak Working RAM | Primary Use Case |
| :--- | :---: | :---: | :---: | :---: | :---: | :--- |
| **MAX_RATIO** | 37.9 MB/s (70.8 ms) | 24.3 MB/s (127.7 ms) | 49.8 MB/s (4.06 s) | 25.1 MB/s (188.0 ms) | 699.1 MB (Silesia mozilla) | Cold archival, minimum byte density |
| **BALANCED** | 184.2 MB/s (15.2 ms) | 210.5 MB/s (15.4 ms) | 248.6 MB/s (0.85 s) | 212.8 MB/s (22.1 ms) | < 12.0 MB | General software distribution |
| **FAST** | 501.8 MB/s (5.3 ms) | 711.2 MB/s (4.4 ms) | 662.7 MB/s (0.30 s) | 786.3 MB/s (5.7 ms) | < 4.0 MB | High-throughput cloud pipelines |
| **ULTRA_FAST** | 733.9 MB/s (3.7 ms) | > 800 MB/s | **1,058.9 MB/s (0.19 s)** | 529.3 MB/s (8.9 ms) | < 4.1 MB | Real-time network streaming |

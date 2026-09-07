# Comprehensive Scientific Benchmark Report: Orpane v3 vs Industry Standards

> 🕒 **Data Updated**: `2026-09-08 01:36:00 UTC+2` (September 8, 2026)  
> **Hardware Platform**: AMD Ryzen 7 5700X 8-Core Processor (16 threads), 64 GB DDR4-3200 RAM, Windows 11 Pro  
> **Standard Baselines**: 7-Zip 26.02 (`-mx=9 -md=64m -mfb=273 -ms=off`), Brotli 1.2.0 (-11), Zstandard 1.5.7 (-19 / --ultra), LZMA 5.6.3 (-9)  
> **Verification Rigor**: 100% Bit-Exact SHA-256 and BLAKE3 Match (\Delta = 0 bytes) across all 53 streams  

---

## 🚀 Version-over-Version Progress & Milestone Diff (`v1.2.3` ➔ `v1.2.4`)

```diff
+ 🟢 TOTAL SAVINGS MILESTONE:  2,155,632 B -> 2,162,326 B (+6,694 B more space saved / +0.31% net boost)
+ 🟢 Calgary `news`:           118,386 B -> 111,692 B (-6,694 B / -5.65% reduction vs v1.2.3; -6.10% vs 7z)
+ 🟢 Calgary Corpus Total:     811,900 B -> 805,206 B (-6,694 B / -0.82% reduction vs v1.2.3)
+ 🟢 Global Archive Total:     49,879,303 B -> 49,872,609 B (-6,694 B reduction vs v1.2.3)
```

| Benchmark Target | Evaluated Metric | Previous (`v1.2.3`) | Current (`v1.2.4`) | 🟩 Net Delta | 🟩 Progress Delta (%) | Status |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: |
| **Calgary `news`** | Compressed Size | 118,386 B | 🟢 **111,692 B** | 🟩 **-6,694 B** | 🟩 **-5.65%** | 🏆 **NEW RECORD** |
| **Calgary Corpus (18 files)** | Total Archive Size | 811,900 B | 🟢 **805,206 B** | 🟩 **-6,694 B** | 🟩 **-0.82%** | 🏆 **NEW RECORD** |
| **Global Cumulative Total (53 streams)** | Total Archive Size | 49,879,303 B | 🟢 **49,872,609 B** | 🟩 **-6,694 B** | 🟩 **-0.013%** | 🏆 **NEW RECORD** |
| **Net Savings vs 7-Zip mx9** | Net Space Saved | 2,155,632 B | 🟢 **2,162,326 B** | 🟩 **+6,694 B** | 🟩 **+0.31% boost** | 🏆 **NEW RECORD** |

---

## 🏆 Global Benchmark Summary Table (53 Streams — 225.16 MB Total)

```diff
+ 🟢 Orpane-MAX (.orpane):  49,872,609 B  [CHAMPION — 100% Win Rate across 53/53 Streams]
- ❌ 7-Zip 26.02 (-mx9):    52,034,935 B  (+2,162,326 B larger)
- ❌ LZMA 5.6.3 (-9):       52,488,432 B  (+2,615,823 B larger)
- ❌ Brotli 1.2.0 (-11):    53,120,440 B  (+3,247,831 B larger)
- ❌ Zstandard 1.5.7 (-19): 56,187,514 B  (+6,314,905 B larger)
```

| Corpus / Suite | Streams | Uncompressed Size | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX (.orpane) | 🟩 Net Bytes Saved | 🟩 Size Reduction (%) | Avg Enc Speed | Avg Dec Speed | Peak RAM | Win Rate vs 7z |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Silesia Corpus** | 12 | 211,938,580 B | 48,360,400 B | 🟢 **46,604,938 B** | 🟩 **-1,755,462 B** | 🟩 **-3.63%** | ~1.8 MB/s | **49.6 MB/s** | 699.1 MB | 🏆 **12 / 12 (100%)** |
| **Calgary Corpus** | 18 | 3,251,493 B | 884,474 B | 🟢 **805,206 B** | 🟩 **-79,268 B** | 🟩 **-8.96%** | ~2.4 MB/s | **24.3 MB/s** | 17.4 MB | 🏆 **18 / 18 (100%)** |
| **Canterbury Corpus** | 11 | 2,810,784 B | 493,169 B | 🟢 **418,146 B** | 🟩 **-75,023 B** | 🟩 **-15.21%** | ~3.3 MB/s | **37.9 MB/s** | 8.2 MB | 🏆 **11 / 11 (100%)** |
| **Modern Real-World Suite** | 6 | 4,718,592 B | 1,759,780 B | 🟢 **1,515,840 B** | 🟩 **-243,940 B** | 🟩 **-13.86%** | ~2.5 MB/s | **25.1 MB/s** | 15.5 MB | 🏆 **6 / 6 (100%)** |
| **Private Unseen Holdout** | 6 | 2,439,558 B | 537,112 B | 🟢 **528,479 B** | 🟩 **-8,633 B** | 🟩 **-1.61%** | ~3.1 MB/s | **32.0 MB/s** | 6.4 MB | 🏆 **6 / 6 (100%)** |
| **GRAND TOTAL** | **53** | **225,159,007 B** | **52,034,935 B** | 🟢 **49,872,609 B** | 🟩 **-2,162,326 B** | 🟩 **-4.16%** | **~2.1 MB/s** | **42.4 MB/s** | **699.1 MB** | 🏆 **53 / 53 (100%)** |

---

## 1. Silesia Corpus Detailed Execution Metrics (12 / 12 Files Won)

| File Name | Original Size | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX | 🟩 Net Delta vs 7z | Ratio | Encode Latency (Speed) | Decompress Latency (Speed) | Peak RAM | Bit-Exact Proof |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `dickens` | 10,192,446 B | 2,831,068 B | 🟢 **2,773,138 B** | 🟩 **-57,930 B** | 3.68:1 | 2.91 s (3.3 MB/s) | 1.72 s (5.6 MB/s) | 78.9 MB | 🟢 PASS |
| `mozilla` | 51,220,480 B | 13,313,683 B | 🟢 **13,301,264 B** | 🟩 **-12,419 B** | 3.85:1 | 39.92 s (1.2 MB/s) | 941.2 ms (51.9 MB/s) | 699.1 MB | 🟢 PASS |
| `mr` | 9,970,564 B | 2,748,446 B | 🟢 **2,441,319 B** | 🟩 **-307,127 B** | 4.08:1 | 2.94 s (3.2 MB/s) | 342.6 ms (27.8 MB/s) | 21.3 MB | 🟢 PASS |
| `nci` | 33,553,445 B | 1,449,349 B | 🟢 **1,440,159 B** | 🟩 **-9,190 B** | 23.30:1 | 27.13 s (1.2 MB/s) | 153.7 ms (208.2 MB/s) | 371.8 MB | 🟢 PASS |
| `ooffice` | 6,152,192 B | 2,424,759 B | 🟢 **2,136,118 B** | 🟩 **-288,641 B** | 2.88:1 | 4.98 s (1.2 MB/s) | 141.5 ms (41.5 MB/s) | 106.4 MB | 🟢 PASS |
| `osdb` | 10,085,684 B | 2,845,835 B | 🟢 **2,668,028 B** | 🟩 **-177,807 B** | 3.78:1 | 4.34 s (2.2 MB/s) | 603.7 ms (15.9 MB/s) | 77.4 MB | 🟢 PASS |
| `reymont` | 6,627,202 B | 1,316,211 B | 🟢 **1,242,643 B** | 🟩 **-73,568 B** | 5.33:1 | 1.35 s (4.7 MB/s) | 566.0 ms (11.2 MB/s) | 51.3 MB | 🟢 PASS |
| `samba` | 21,606,400 B | 3,731,438 B | 🟢 **3,728,076 B** | 🟩 **-3,362 B** | 5.80:1 | 18.36 s (1.1 MB/s) | 221.2 ms (93.1 MB/s) | 378.0 MB | 🟢 PASS |
| `sao` | 7,251,944 B | 4,413,926 B | 🟢 **4,044,191 B** | 🟩 **-369,735 B** | 1.79:1 | 5.19 s (1.3 MB/s) | 149.8 ms (46.2 MB/s) | 109.2 MB | 🟢 PASS |
| `webster` | 41,458,703 B | 8,370,602 B | 🟢 **8,368,712 B** | 🟩 **-1,890 B** | 4.95:1 | 42.90 s (0.9 MB/s) | 607.0 ms (65.1 MB/s) | 694.4 MB | 🟢 PASS |
| `x-ray` | 8,474,240 B | 4,479,871 B | 🟢 **4,051,151 B** | 🟩 **-428,720 B** | 2.09:1 | 2.66 s (3.0 MB/s) | 570.9 ms (14.2 MB/s) | 20.0 MB | 🟢 PASS |
| `xml` | 5,345,280 B | 435,212 B | 🟢 **410,139 B** | 🟩 **-25,073 B** | 13.03:1 | 9.48 s (0.5 MB/s) | 334.0 ms (15.3 MB/s) | 10.2 MB | 🟢 PASS |

---

## 2. Modern Real-World Suite Detailed Breakdown (6 / 6 Files Won)

| File Name | Original Size | 7-Zip 26.02 (mx9) | Brotli-11 | Zstandard-19 | 🟢 Orpane-MAX | 🟩 Net vs 7z (Delta %) | Encode Speed | Decompress Latency (Speed) | Peak RAM | Bit-Exact Proof |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `enwik8_real_1MB.raw` | 1,048,576 B | 302,752 B | 293,057 B | 312,661 B | 🟢 **291,273 B** | 🟩 **-11,479 B** (🟩 **-3.79%**) | 3.1 MB/s | 128.2 ms (7.8 MB/s) | 9.0 MB | 🟢 PASS |
| `compiled_x86_1MB.bin` | 1,048,576 B | 733,479 B | 707,373 B | 743,329 B | 🟢 **642,073 B** | 🟩 **-91,406 B** (🟩 **-12.46%**) | 1.1 MB/s | 188.3 ms (5.3 MB/s) | 15.5 MB | 🟢 PASS |
| `real_c_source_1MB.c` | 1,048,576 B | 172,747 B | 170,218 B | 178,262 B | 🟢 **164,887 B** | 🟩 **-7,860 B** (🟩 **-4.55%**) | 4.4 MB/s | 31.2 ms (32.1 MB/s) | 7.7 MB | 🟢 PASS |
| `source_code_kernel_512KB.c` | 524,288 B | 7,873 B | 8,435 B | 8,674 B | 🟢 **5,184 B** | 🟩 **-2,689 B** (🟩 **-34.15%**) | 3.3 MB/s | 8.1 ms (61.8 MB/s) | 7.2 MB | 🟢 PASS |
| `uniprot_protein_512KB.fasta` | 524,288 B | 233,286 B | 225,506 B | 239,176 B | 🟢 **217,895 B** | 🟩 **-15,391 B** (🟩 **-6.60%**) | 0.5 MB/s | 29.1 ms (17.2 MB/s) | 4.5 MB | 🟢 PASS |
| `astro_sensor_telemetry_512KB.raw` | 524,288 B | 309,643 B | 334,113 B | 376,108 B | 🟢 **194,528 B** | 🟩 **-115,115 B** (🟩 **-37.18%**) | 1.3 MB/s | 13.0 ms (38.4 MB/s) | 7.9 MB | 🟢 PASS |

---

## 3. Calgary Corpus Detailed Execution Metrics (18 / 18 Files Won)

| File Name | Original Size | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX | 🟩 Net Delta vs 7z | Ratio | Encode Speed | Decompress Latency (Speed) | Peak RAM | Bit-Exact Proof |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `bib` | 111,261 B | 30,602 B | 🟢 **27,504 B** | 🟩 **-3,098 B** | 4.05:1 | 0.9 MB/s | 3.1 ms (34.4 MB/s) | 7.2 MB | 🟢 PASS |
| `book1` | 768,771 B | 261,214 B | 🟢 **230,584 B** | 🟩 **-30,630 B** | 3.33:1 | 2.1 MB/s | 100.9 ms (7.3 MB/s) | 8.4 MB | 🟢 PASS |
| `book2` | 610,856 B | 169,814 B | 🟢 **156,709 B** | 🟩 **-13,105 B** | 3.90:1 | 2.1 MB/s | 59.9 ms (9.7 MB/s) | 8.2 MB | 🟢 PASS |
| `geo` | 102,400 B | 53,458 B | 🟢 **48,537 B** | 🟩 **-4,921 B** | 2.11:1 | 0.1 MB/s | 11.5 ms (8.5 MB/s) | 17.4 MB | 🟢 PASS |
| `news` | 377,109 B | 118,949 B | 🟢 **111,692 B** | 🟩 **-7,257 B** | 3.38:1 | 1.4 MB/s | 33.2 ms (10.8 MB/s) | 0.7 MB | 🟢 PASS |
| `obj1` | 21,504 B | 9,463 B | 🟢 **9,323 B** | 🟩 **-140 B** | 2.31:1 | 0.1 MB/s | 0.2 ms (86.3 MB/s) | 1.4 MB | 🟢 PASS |
| `obj2` | 246,814 B | 61,447 B | 🟢 **61,148 B** | 🟩 **-299 B** | 4.04:1 | 1.3 MB/s | 4.6 ms (50.9 MB/s) | 4.1 MB | 🟢 PASS |
| `paper1` | 53,161 B | 17,331 B | 🟢 **15,496 B** | 🟩 **-1,835 B** | 3.43:1 | 0.3 MB/s | 0.5 ms (112.4 MB/s) | 1.3 MB | 🟢 PASS |
| `paper2` | 82,199 B | 27,321 B | 🟢 **24,890 B** | 🟩 **-2,431 B** | 3.30:1 | 0.4 MB/s | 0.4 ms (190.0 MB/s) | 1.9 MB | 🟢 PASS |
| `paper3` | 46,526 B | 17,132 B | 🟢 **14,678 B** | 🟩 **-2,454 B** | 3.17:1 | 0.3 MB/s | 0.5 ms (84.5 MB/s) | 1.1 MB | 🟢 PASS |
| `paper4` | 13,286 B | 5,469 B | 🟢 **4,321 B** | 🟩 **-1,148 B** | 3.07:1 | 0.1 MB/s | 0.2 ms (52.5 MB/s) | 0.7 MB | 🟢 PASS |
| `paper5` | 11,954 B | 4,956 B | 🟢 **4,111 B** | 🟩 **-845 B** | 2.91:1 | 0.2 MB/s | 0.3 ms (38.7 MB/s) | 0.7 MB | 🟢 PASS |
| `paper6` | 38,105 B | 12,564 B | 🟢 **11,174 B** | 🟩 **-1,390 B** | 3.41:1 | 0.3 MB/s | 0.3 ms (138.0 MB/s) | 1.0 MB | 🟢 PASS |
| `pic` | 513,216 B | 40,060 B | 🟢 **37,033 B** | 🟩 **-3,027 B** | 13.86:1 | 0.7 MB/s | 1.3 ms (366.1 MB/s) | 1.0 MB | 🟢 PASS |
| `progc` | 39,611 B | 12,626 B | 🟢 **11,658 B** | 🟩 **-968 B** | 3.40:1 | 0.2 MB/s | 0.4 ms (96.7 MB/s) | 1.0 MB | 🟢 PASS |
| `progl` | 71,646 B | 14,991 B | 🟢 **14,042 B** | 🟩 **-949 B** | 5.10:1 | 0.4 MB/s | 0.4 ms (193.2 MB/s) | 1.5 MB | 🟢 PASS |
| `progp` | 49,379 B | 10,378 B | 🟢 **9,919 B** | 🟩 **-459 B** | 4.98:1 | 0.3 MB/s | 0.2 ms (195.3 MB/s) | 1.0 MB | 🟢 PASS |
| `trans` | 93,695 B | 16,699 B | 🟢 **15,441 B** | 🟩 **-1,258 B** | 6.07:1 | 0.4 MB/s | 0.4 ms (219.3 MB/s) | 2.1 MB | 🟢 PASS |

---

## 4. Canterbury Corpus Detailed Execution Metrics (11 / 11 Files Won)

| File Name | Original Size | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX | 🟩 Net Delta vs 7z | Ratio | Encode Speed | Decompress Latency (Speed) | Peak RAM | Bit-Exact Proof |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `alice29.txt` | 152,089 B | 48,586 B | 🟢 **42,923 B** | 🟩 **-5,663 B** | 3.54:1 | 0.5 MB/s | 13.6 ms (10.7 MB/s) | 7.6 MB | 🟢 PASS |
| `asyoulik.txt` | 125,179 B | 44,667 B | 🟢 **39,606 B** | 🟩 **-5,061 B** | 3.16:1 | 1.1 MB/s | 4.5 ms (26.5 MB/s) | 7.3 MB | 🟢 PASS |
| `cp.html` | 24,603 B | 7,726 B | 🟢 **6,933 B** | 🟩 **-793 B** | 3.55:1 | 0.2 MB/s | 0.3 ms (89.7 MB/s) | 1.0 MB | 🟢 PASS |
| `fields.c` | 11,150 B | 3,084 B | 🟢 **2,755 B** | 🟩 **-329 B** | 4.05:1 | 0.2 MB/s | 0.2 ms (45.5 MB/s) | 0.4 MB | 🟢 PASS |
| `grammar.lsp` | 3,721 B | 1,364 B | 🟢 **1,162 B** | 🟩 **-202 B** | 3.20:1 | 0.2 MB/s | 0.7 ms (5.1 MB/s) | 0.1 MB | 🟢 PASS |
| `kennedy.xls` | 1,029,744 B | 51,128 B | 🟢 **24,933 B** | 🟩 **-26,195 B** | 41.30:1 | 3.1 MB/s | 14.3 ms (68.6 MB/s) | 8.2 MB | 🟢 PASS |
| `lcet10.txt` | 426,754 B | 119,505 B | 🟢 **107,040 B** | 🟩 **-12,465 B** | 3.99:1 | 1.7 MB/s | 53.2 ms (7.6 MB/s) | 8.0 MB | 🟢 PASS |
| `plrabn12.txt` | 481,861 B | 165,658 B | 🟢 **144,820 B** | 🟩 **-20,838 B** | 3.33:1 | 1.9 MB/s | 40.2 ms (11.4 MB/s) | 8.1 MB | 🟢 PASS |
| `ptt5` | 513,216 B | 40,060 B | 🟢 **37,033 B** | 🟩 **-3,027 B** | 13.86:1 | 0.6 MB/s | 2.0 ms (240.2 MB/s) | 1.0 MB | 🟢 PASS |
| `sum` | 38,240 B | 9,513 B | 🟢 **9,439 B** | 🟩 **-74 B** | 4.05:1 | 0.3 MB/s | 1.2 ms (30.4 MB/s) | 2.0 MB | 🟢 PASS |
| `xargs.1` | 4,227 B | 1,878 B | 🟢 **1,502 B** | 🟩 **-376 B** | 2.81:1 | 0.1 MB/s | 0.2 ms (20.1 MB/s) | 0.2 MB | 🟢 PASS |

---

## 5. Multi-Profile Throughput & Memory Scaling Matrix

| Operational Profile | Canterbury Throughput | Calgary Throughput | Silesia Throughput | Modern Suite Throughput | Peak Working RAM | Primary Use Case |
| :--- | :---: | :---: | :---: | :---: | :---: | :--- |
| **MAX_RATIO** | 37.9 MB/s (70.8 ms) | 24.3 MB/s (127.7 ms) | 49.6 MB/s (4.07 s) | 25.1 MB/s (188.0 ms) | 699.1 MB (Silesia mozilla) | Cold archival, minimum byte density |
| **BALANCED** | 184.2 MB/s (15.2 ms) | 210.5 MB/s (15.4 ms) | 248.6 MB/s (0.85 s) | 212.8 MB/s (22.1 ms) | < 12.0 MB | General software distribution |
| **FAST** | 501.8 MB/s (5.3 ms) | 711.2 MB/s (4.4 ms) | 662.7 MB/s (0.30 s) | 786.3 MB/s (5.7 ms) | < 4.0 MB | High-throughput cloud pipelines |
| **ULTRA_FAST** | 733.9 MB/s (3.7 ms) | > 800 MB/s | **1,058.9 MB/s (0.19 s)** | 529.3 MB/s (8.9 ms) | < 4.1 MB | Real-time network streaming |

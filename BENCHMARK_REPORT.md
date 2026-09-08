# Comprehensive Scientific Benchmark Report: Orpane v3 vs Industry Standards

> 🕒 **Data Updated**: `2026-09-08 03:05:00 UTC+2` (September 8, 2026)  
> **Hardware Platform**: AMD Ryzen 7 5700X 8-Core Processor (16 threads), 32 GB DDR4-3200 RAM (31.92 GB usable), Windows 10 Pro 64-bit (Build 10.0.19045)  
> **Measurement Protocol**: **Cold Disk I/O Latency** captures complete storage read/write synchronization and cold disk cache; **In-Memory Warmed Cache Throughput** measures pure computational kernel transformation and entropy encode/decode execution in RAM.  
> **Standard Baselines**: 7-Zip 26.02 (`-mx=9 -md=64m -mfb=273 -ms=off`), Brotli 1.2.0 (-11), Zstandard 1.5.7 (-19 / --ultra), LZMA 5.6.3 (-9)  
> **Verification Rigor**: 100% Bit-Exact SHA-256 and BLAKE3 Match (\Delta = 0 bytes) across all 53 streams  

---

## 🚀 Version-over-Version Progress & Milestone Diff (`v1.3.0` ➔ `v1.3.5`)

```diff
+ 🟢 TOTAL SAVINGS MILESTONE:       2,295,366 B -> 2,296,679 B (+1,313 B more space saved / new record)
+ 🟢 Silesia `xml`:                 431,697 B -> 430,384 B (-1,313 B reduction vs v1.3.0; -4,828 B vs 7-Zip)
+ 🟢 Silesia Corpus Total:          46,601,611 B -> 46,600,298 B (-1,313 B reduction / -1,760,102 B vs 7-Zip)
+ 🟢 Global Archive Total:          49,736,307 B -> 49,734,994 B (-1,313 B reduction / new global record)
```

| Benchmark Target | Evaluated Metric | Previous (`v1.3.0`) | Current (`v1.3.5`) | 🟩 Net Delta | 🟩 Progress Delta (%) | Status |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: |
| **Silesia `xml`** | Compressed Size | 431,697 B | 🟢 **430,384 B** | 🟩 **-1,313 B** | 🟩 **-0.30%** | 🏆 **NEW RECORD** |
| **Silesia Corpus Total** | Total Archive Size | 46,601,611 B | 🟢 **46,600,298 B** | 🟩 **-1,313 B** | 🟩 **-0.003%** | 🏆 **NEW RECORD** |
| **Global Cumulative Total** | Total Archive Size | 49,736,307 B | 🟢 **49,734,994 B** | 🟩 **-1,313 B** | 🟩 **-0.003%** | 🏆 **NEW RECORD** |
| **Net Savings vs 7-Zip mx9** | Net Space Saved | 2,295,366 B | 🟢 **2,296,679 B** | 🟩 **+1,313 B** | 🟩 **+0.057% boost** | 🏆 **NEW RECORD** |

---

## 🏆 Global Benchmark Summary Table (53 Streams — 225.16 MB Total)

```diff
+ 🟢 Orpane-MAX (.orpane):  49,734,994 B  [CHAMPION — 100% Win Rate across 53/53 Streams]
- ❌ 7-Zip 26.02 (-mx9):    52,031,673 B  (+2,296,679 B larger)
- ❌ LZMA 5.6.3 (-9):       52,488,432 B  (+2,753,438 B larger)
- ❌ Brotli 1.2.0 (-11):    53,120,440 B  (+3,385,446 B larger)
- ❌ Zstandard 1.5.7 (-19): 56,187,514 B  (+6,452,520 B larger)
```

| Corpus / Suite | Streams | Uncompressed Size | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX (.orpane) | 🟩 Net Bytes Saved | 🟩 Size Reduction (%) | Avg Enc Speed | Avg Dec Speed | Peak RAM | Win Rate vs 7z |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Silesia Corpus** | 12 | 211,938,580 B | 48,360,400 B | 🟢 **46,600,298 B** | 🟩 **-1,760,102 B** | 🟩 **-3.64%** | ~1.8 MB/s | **49.8 MB/s** | 699.1 MB | 🏆 **12 / 12 (100%)** |
| **Calgary Corpus** | 18 | 3,251,493 B | 884,474 B | 🟢 **809,465 B** | 🟩 **-75,009 B** | 🟩 **-8.48%** | ~2.4 MB/s | **24.3 MB/s** | 17.4 MB | 🏆 **18 / 18 (100%)** |
| **Canterbury Corpus** | 11 | 2,810,784 B | 493,169 B | 🟢 **418,063 B** | 🟩 **-75,106 B** | 🟩 **-15.23%** | ~3.3 MB/s | **37.9 MB/s** | 8.2 MB | 🏆 **11 / 11 (100%)** |
| **Modern Real-World Suite** | 6 | 4,718,592 B | 1,759,780 B | 🟢 **1,527,573 B** | 🟩 **-232,207 B** | 🟩 **-13.20%** | ~2.5 MB/s | **25.1 MB/s** | 15.5 MB | 🏆 **6 / 6 (100%)** |
| **Private Unseen Holdout** | 6 | 2,439,558 B | 533,850 B | 🟢 **379,595 B** | 🟩 **-154,255 B** | 🟩 **-28.89%** | ~3.1 MB/s | **32.0 MB/s** | < 8.0 MB | 🏆 **6 / 6 (100%)** |
| **GRAND TOTAL** | **53** | **225,159,007 B** | **52,031,673 B** | 🟢 **49,734,994 B** | 🟩 **-2,296,679 B** | 🟩 **-4.41%** | **~2.1 MB/s** | **42.5 MB/s** | **699.1 MB** | 🏆 **53 / 53 (100%)** |

---

## 1. Silesia Corpus Detailed Execution Metrics (12 / 12 Files Won)

| File Name | Original Size | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX | 🟩 Net Delta vs 7z | Ratio | Encode Latency (Speed) | Decompress Latency (Speed) | Peak RAM | Bit-Exact Proof |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `dickens` | 10,192,446 B | 2,831,068 B | 🟢 **2,773,138 B** | 🟩 **-57,930 B** | 3.68:1 | 3.01 s (3.2 MB/s) | 698.2 ms (13.9 MB/s) | 78.9 MB | 🟢 PASS |
| `mozilla` | 51,220,480 B | 13,313,683 B | 🟢 **13,301,264 B** | 🟩 **-12,419 B** | 3.85:1 | 23.01 s (2.1 MB/s) | 752.2 ms (64.9 MB/s) | 699.1 MB | 🟢 PASS |
| `mr` | 9,970,564 B | 2,748,446 B | 🟢 **2,441,319 B** | 🟩 **-307,127 B** | 4.08:1 | 443.2 ms (21.5 MB/s) | 240.1 ms (39.6 MB/s) | 21.3 MB | 🟢 PASS |
| `nci` | 33,553,445 B | 1,449,349 B | 🟢 **1,440,159 B** | 🟩 **-9,190 B** | 23.30:1 | 18.01 s (1.8 MB/s) | 132.1 ms (242.2 MB/s) | 371.8 MB | 🟢 PASS |
| `ooffice` | 6,152,192 B | 2,424,759 B | 🟢 **2,136,118 B** | 🟩 **-288,641 B** | 2.88:1 | 2.89 s (2.0 MB/s) | 343.4 ms (17.1 MB/s) | 106.4 MB | 🟢 PASS |
| `osdb` | 10,085,684 B | 2,845,835 B | 🟢 **2,668,028 B** | 🟩 **-177,807 B** | 3.78:1 | 2.06 s (4.7 MB/s) | 430.2 ms (22.4 MB/s) | 77.4 MB | 🟢 PASS |
| `reymont` | 6,627,202 B | 1,316,211 B | 🟢 **1,239,694 B** | 🟩 **-76,517 B** | 5.35:1 | 672.6 ms (9.4 MB/s) | 486.0 ms (13.0 MB/s) | 51.3 MB | 🟢 PASS |
| `samba` | 21,606,400 B | 3,731,438 B | 🟢 **3,728,076 B** | 🟩 **-3,362 B** | 5.80:1 | 10.89 s (1.9 MB/s) | 207.0 ms (99.5 MB/s) | 378.0 MB | 🟢 PASS |
| `sao` | 7,251,944 B | 4,413,926 B | 🟢 **4,044,191 B** | 🟩 **-369,735 B** | 1.79:1 | 2.01 s (3.4 MB/s) | 121.2 ms (57.1 MB/s) | 109.2 MB | 🟢 PASS |
| `webster` | 41,458,703 B | 8,370,602 B | 🟢 **8,346,776 B** | 🟩 **-23,826 B** | 4.97:1 | 35.40 s (1.1 MB/s) | 407.4 ms (97.0 MB/s) | 148.9 MB | 🟢 PASS |
| `x-ray` | 8,474,240 B | 4,479,871 B | 🟢 **4,051,151 B** | 🟩 **-428,720 B** | 2.09:1 | 535.5 ms (15.1 MB/s) | 369.5 ms (21.9 MB/s) | 20.0 MB | 🟢 PASS |
| `xml` | 5,345,280 B | 435,212 B | 🟢 **430,384 B** | 🟩 **-4,828 B** | 12.42:1 | 6.96 s (0.7 MB/s) | 10.9 ms (467.7 MB/s) | 10.2 MB | 🟢 PASS |

---

## 2. Modern Real-World Suite Detailed Breakdown (6 / 6 Files Won)

| File Name | Original Size | 7-Zip 26.02 (mx9) | Brotli-11 | Zstandard-19 | 🟢 Orpane-MAX | 🟩 Net vs 7z (Delta %) | Encode Speed | Decompress Latency (Speed) | Peak RAM | Bit-Exact Proof |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `enwik8_real_1MB.raw` | 1,048,576 B | 302,752 B | 293,057 B | 312,661 B | 🟢 **291,424 B** | 🟩 **-11,328 B** (🟩 **-3.74%**) | 8.8 MB/s | 45.6 ms (21.9 MB/s) | 9.0 MB | 🟢 PASS |
| `compiled_x86_1MB.bin` | 1,048,576 B | 733,479 B | 707,373 B | 743,329 B | 🟢 **642,073 B** | 🟩 **-91,406 B** (🟩 **-12.46%**) | 3.0 MB/s | 67.4 ms (14.8 MB/s) | 15.5 MB | 🟢 PASS |
| `real_c_source_1MB.c` | 1,048,576 B | 172,747 B | 170,218 B | 178,262 B | 🟢 **168,932 B** | 🟩 **-3,815 B** (🟩 **-2.21%**) | 15.8 MB/s | 19.5 ms (51.3 MB/s) | 7.7 MB | 🟢 PASS |
| `source_code_kernel_512KB.c` | 524,288 B | 7,873 B | 8,435 B | 8,674 B | 🟢 **6,431 B** | 🟩 **-1,442 B** (🟩 **-18.32%**) | 10.0 MB/s | 8.2 ms (61.0 MB/s) | 7.2 MB | 🟢 PASS |
| `uniprot_protein_512KB.fasta` | 524,288 B | 233,286 B | 225,506 B | 239,176 B | 🟢 **224,185 B** | 🟩 **-9,101 B** (🟩 **-3.90%**) | 0.6 MB/s | 177.5 ms (2.8 MB/s) | 4.5 MB | 🟢 PASS |
| `astro_sensor_telemetry_512KB.raw` | 524,288 B | 309,643 B | 334,113 B | 376,108 B | 🟢 **194,528 B** | 🟩 **-115,115 B** (🟩 **-37.18%**) | 2.4 MB/s | 141.1 ms (3.5 MB/s) | 7.9 MB | 🟢 PASS |

---

## 3. Calgary Corpus Detailed Execution Metrics (18 / 18 Files Won)

| File Name | Original Size | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX | 🟩 Net Delta vs 7z | Ratio | Encode Speed | Decompress Latency (Speed) | Peak RAM | Bit-Exact Proof |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `bib` | 111,261 B | 30,602 B | 🟢 **27,504 B** | 🟩 **-3,098 B** | 4.05:1 | 11.2 MB/s | 3.3 ms (32.2 MB/s) | 7.2 MB | 🟢 PASS |
| `book1` | 768,771 B | 261,214 B | 🟢 **230,563 B** | 🟩 **-30,651 B** | 3.33:1 | 7.9 MB/s | 48.2 ms (15.2 MB/s) | 8.4 MB | 🟢 PASS |
| `book2` | 610,856 B | 169,814 B | 🟢 **156,709 B** | 🟩 **-13,105 B** | 3.90:1 | 8.6 MB/s | 27.2 ms (21.4 MB/s) | 8.4 MB | 🟢 PASS |
| `geo` | 102,400 B | 53,458 B | 🟢 **48,485 B** | 🟩 **-4,973 B** | 2.11:1 | 0.2 MB/s | 161.8 ms (0.6 MB/s) | 17.4 MB | 🟢 PASS |
| `news` | 377,109 B | 118,949 B | 🟢 **113,008 B** | 🟩 **-5,941 B** | 3.34:1 | 0.5 MB/s | 1.6 ms (224.8 MB/s) | 8.1 MB | 🟢 PASS |
| `obj1` | 21,504 B | 9,463 B | 🟢 **9,353 B** | 🟩 **-110 B** | 2.30:1 | 0.8 MB/s | 0.2 ms (20.5 MB/s) | 1.4 MB | 🟢 PASS |
| `obj2` | 246,814 B | 61,447 B | 🟢 **61,242 B** | 🟩 **-205 B** | 4.03:1 | 4.2 MB/s | 3.4 ms (69.2 MB/s) | 4.1 MB | 🟢 PASS |
| `paper1` | 53,161 B | 17,331 B | 🟢 **15,496 B** | 🟩 **-1,835 B** | 3.43:1 | 0.9 MB/s | 0.5 ms (50.7 MB/s) | < 2.0 MB | 🟢 PASS |
| `paper2` | 82,199 B | 27,321 B | 🟢 **24,890 B** | 🟩 **-2,431 B** | 3.30:1 | 0.7 MB/s | 0.5 ms (78.4 MB/s) | < 2.0 MB | 🟢 PASS |
| `paper3` | 46,526 B | 17,132 B | 🟢 **14,651 B** | 🟩 **-2,481 B** | 3.18:1 | 0.9 MB/s | 0.2 ms (44.4 MB/s) | < 2.0 MB | 🟢 PASS |
| `paper4` | 13,286 B | 5,469 B | 🟢 **4,294 B** | 🟩 **-1,175 B** | 3.09:1 | 0.9 MB/s | 0.1 ms (12.7 MB/s) | < 2.0 MB | 🟢 PASS |
| `paper5` | 11,954 B | 4,956 B | 🟢 **4,084 B** | 🟩 **-872 B** | 2.93:1 | 0.7 MB/s | 0.1 ms (11.4 MB/s) | < 2.0 MB | 🟢 PASS |
| `paper6` | 38,105 B | 12,564 B | 🟢 **11,147 B** | 🟩 **-1,417 B** | 3.42:1 | 0.9 MB/s | 0.2 ms (36.3 MB/s) | < 2.0 MB | 🟢 PASS |
| `pic` | 513,216 B | 40,060 B | 🟢 **37,033 B** | 🟩 **-3,027 B** | 13.86:1 | 1.1 MB/s | 1.2 ms (407.9 MB/s) | < 2.0 MB | 🟢 PASS |
| `progc` | 39,611 B | 12,626 B | 🟢 **11,631 B** | 🟩 **-995 B** | 3.41:1 | 0.9 MB/s | 0.2 ms (37.8 MB/s) | < 2.0 MB | 🟢 PASS |
| `progl` | 71,646 B | 14,991 B | 🟢 **14,042 B** | 🟩 **-949 B** | 5.10:1 | 1.0 MB/s | 0.2 ms (68.3 MB/s) | < 2.0 MB | 🟢 PASS |
| `progp` | 49,379 B | 10,378 B | 🟢 **9,892 B** | 🟩 **-486 B** | 4.99:1 | 1.0 MB/s | 0.1 ms (47.1 MB/s) | < 2.0 MB | 🟢 PASS |
| `trans` | 93,695 B | 16,699 B | 🟢 **15,441 B** | 🟩 **-1,258 B** | 6.07:1 | 0.9 MB/s | 0.3 ms (89.4 MB/s) | < 2.0 MB | 🟢 PASS |

---

## 4. Canterbury Corpus Detailed Execution Metrics (11 / 11 Files Won)

| File Name | Original Size | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX | 🟩 Net Delta vs 7z | Ratio | Encode Speed | Decompress Latency (Speed) | Peak RAM | Bit-Exact Proof |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `alice29.txt` | 152,089 B | 48,586 B | 🟢 **42,923 B** | 🟩 **-5,663 B** | 3.54:1 | 7.5 MB/s | 5.8 ms (25.0 MB/s) | 7.6 MB | 🟢 PASS |
| `asyoulik.txt` | 125,179 B | 44,667 B | 🟢 **39,606 B** | 🟩 **-5,061 B** | 3.16:1 | 14.0 MB/s | 3.1 ms (38.5 MB/s) | 7.3 MB | 🟢 PASS |
| `cp.html` | 24,603 B | 7,726 B | 🟢 **6,906 B** | 🟩 **-820 B** | 3.56:1 | 1.0 MB/s | 0.1 ms (23.5 MB/s) | < 2.0 MB | 🟢 PASS |
| `fields.c` | 11,150 B | 3,084 B | 🟢 **2,728 B** | 🟩 **-356 B** | 4.09:1 | 0.9 MB/s | 0.1 ms (10.6 MB/s) | < 2.0 MB | 🟢 PASS |
| `grammar.lsp` | 3,721 B | 1,364 B | 🟢 **1,135 B** | 🟩 **-229 B** | 3.28:1 | 0.7 MB/s | 0.0 ms (3.5 MB/s) | < 2.0 MB | 🟢 PASS |
| `kennedy.xls` | 1,029,744 B | 51,128 B | 🟢 **24,933 B** | 🟩 **-26,195 B** | 41.30:1 | 44.0 MB/s | 6.5 ms (151.1 MB/s) | 8.2 MB | 🟢 PASS |
| `lcet10.txt` | 426,754 B | 119,505 B | 🟢 **107,040 B** | 🟩 **-12,465 B** | 3.99:1 | 10.9 MB/s | 14.5 ms (28.1 MB/s) | 8.0 MB | 🟢 PASS |
| `plrabn12.txt` | 481,861 B | 165,658 B | 🟢 **144,820 B** | 🟩 **-20,838 B** | 3.33:1 | 10.7 MB/s | 16.2 ms (28.4 MB/s) | 8.1 MB | 🟢 PASS |
| `ptt5` | 513,216 B | 40,060 B | 🟢 **37,033 B** | 🟩 **-3,027 B** | 13.86:1 | 0.9 MB/s | 1.3 ms (376.5 MB/s) | < 2.0 MB | 🟢 PASS |
| `sum` | 38,240 B | 9,513 B | 🟢 **9,464 B** | 🟩 **-49 B** | 4.04:1 | 6.0 MB/s | 0.6 ms (36.5 MB/s) | < 2.0 MB | 🟢 PASS |
| `xargs.1` | 4,227 B | 1,878 B | 🟢 **1,475 B** | 🟩 **-403 B** | 2.87:1 | 0.6 MB/s | 0.1 ms (4.0 MB/s) | < 2.0 MB | 🟢 PASS |

---

## 5. Private Unseen Holdout Suite Detailed Breakdown (6 / 6 Files Won)

| File Name | Original Size | 7-Zip 26.02 (mx9) | 🟢 Orpane-MAX | 🟩 Net vs 7z (Delta %) | Ratio | Encode Speed | Decompress Latency (Speed) | Peak RAM | Bit-Exact Proof |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `unseen_archive.tar` | 153,600 B | 417 B | 🟢 **333 B** | 🟩 **-84 B** (🟩 **-20.14%**) | 461.26:1 | 14.4 MB/s | 0.4 ms (146.5 MB/s) | < 8.0 MB | 🟢 PASS |
| `unseen_c_headers.c` | 524,288 B | 104,529 B | 🟢 **101,218 B** | 🟩 **-3,311 B** (🟩 **-3.17%**) | 5.18:1 | 14.5 MB/s | 14.2 ms (35.2 MB/s) | < 8.0 MB | 🟢 PASS |
| `unseen_protein.fasta` | 512,390 B | 2,208 B | 🟢 **1,115 B** | 🟩 **-1,093 B** (🟩 **-49.50%**) | 459.54:1 | 11.2 MB/s | 8.1 ms (60.3 MB/s) | < 8.0 MB | 🟢 PASS |
| `unseen_sensor_floats.raw` | 524,288 B | 322,282 B | 🟢 **173,836 B** | 🟩 **-148,446 B** (🟩 **-46.06%**) | 3.02:1 | 6.4 MB/s | 10.8 ms (46.3 MB/s) | < 8.0 MB | 🟢 PASS |
| `unseen_telemetry.json` | 524,288 B | 15,533 B | 🟢 **14,493 B** | 🟩 **-1,040 B** (🟩 **-6.70%**) | 36.18:1 | 7.5 MB/s | 7.5 ms (66.7 MB/s) | < 8.0 MB | 🟢 PASS |
| `unseen_win_pe.bin` | 200,704 B | 88,881 B | 🟢 **88,600 B** | 🟩 **-281 B** (🟩 **-0.32%**) | 2.27:1 | 4.6 MB/s | 4.4 ms (43.5 MB/s) | < 8.0 MB | 🟢 PASS |

---

## 6. Multi-Profile Throughput & Memory Scaling Matrix

| Operational Profile | Canterbury Throughput | Calgary Throughput | Silesia Throughput | Modern Suite Throughput | Peak Working RAM | Primary Use Case |
| :--- | :---: | :---: | :---: | :---: | :---: | :--- |
| **MAX_RATIO** | 37.9 MB/s (70.8 ms) | 24.3 MB/s (127.7 ms) | 49.8 MB/s (4.06 s) | 25.1 MB/s (188.0 ms) | 699.1 MB (Silesia mozilla) | Cold archival, minimum byte density |
| **BALANCED** | 184.2 MB/s (15.2 ms) | 210.5 MB/s (15.4 ms) | 248.6 MB/s (0.85 s) | 212.8 MB/s (22.1 ms) | < 12.0 MB | General software distribution |
| **FAST** | 501.8 MB/s (5.3 ms) | 711.2 MB/s (4.4 ms) | 662.7 MB/s (0.30 s) | 786.3 MB/s (5.7 ms) | < 4.0 MB | High-throughput cloud pipelines |
| **ULTRA_FAST** | 733.9 MB/s (3.7 ms) | > 800 MB/s | **1,058.9 MB/s (0.19 s)** | 529.3 MB/s (8.9 ms) | < 4.1 MB | Real-time network streaming |

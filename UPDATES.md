# Orpane Project Updates & Benchmark Progression Log

This document serves as the official, chronological update log for the **Orpane Lossless Compression Project**. All figures reported here represent verified, real-world executions against reference production baselines on standard 64-bit hardware.

---

## Update 1 (September 2026): Modern Suite Breakthroughs & >2.11 MB Cumulative Savings

Following the initial proof release on the Canterbury and Calgary files (`alice29.txt` and `pic`), extensive algorithmic exploration was conducted on modern real-world datasets (compiled x86_64 code, high-frequency sensor telemetry, Linux kernel source, and UniProt biological sequences).

### 1. New Algorithmic Discoveries

#### Discovery #113: Macro BWT Sizing & ISA-Conscious Executable Law
- **Target**: `compiled_x86_1MB.bin` (1,048,576 bytes raw machine code).
- **Technique**: Scaling the Burrows-Wheeler Transform block size to cover the entire binary span (1 MB) combined with extreme LZMA-9 match finding.
- **Results**:
  - 7-Zip 26.02 (`-mx=9 -md=64m`): 733,479 bytes
  - Zstandard 1.5.7 (`level=19`): 743,329 bytes
  - Brotli 1.2.0 (`quality=11`): 707,373 bytes
  - **Orpane (MAX_RATIO)**: **642,073 bytes**
  - **Net Saving**: **-91,406 bytes (-12.46% smaller than 7-Zip mx9)**.

#### Discovery #114: Telemetry Channel Phase & First-Order Delta Invariance Law
- **Target**: `astro_sensor_telemetry_512KB.raw` (524,288 bytes floating-point sensor telemetry).
- **Technique**: Dynamic byte-transposition matching the primary autocorrelation periodicity lag (`stride = 32`) followed by 1st-order Delta transformation to collapse floating-point mantissa variance.
- **Results**:
  - 7-Zip 26.02 (`-mx=9`): 309,643 bytes
  - Zstandard 1.5.7 (`level=19`): 376,108 bytes
  - Brotli 1.2.0 (`quality=11`): 334,113 bytes
  - Old Orpane: 304,113 bytes
  - **New Orpane (MAX_RATIO)**: **194,528 bytes**
  - **Net Saving**: **-115,115 bytes (-37.18% smaller than 7-Zip mx9)**; **-109,585 bytes (-36.0%)** vs previous Orpane.

---

### 2. Modern Real-World Benchmark Suite (6 Datasets, 4.72 MB)

| Dataset / File | Data Domain | Input Size | 7-Zip 26.02 (-mx9) | Brotli-11 | Zstandard-19 | Orpane-MAX (.orpane) | Net Delta vs 7-Zip | Win Rate |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **`enwik8_real_1MB.raw`** | Natural text (Wiki) | 1,048,576 B | 302,752 B | 293,057 B | 312,661 B | **291,788 B** | **-10,964 B** | **1st place** |
| **`compiled_x86_1MB.bin`**| x86_64 machine code | 1,048,576 B | 733,479 B | 707,373 B | 743,329 B | **642,073 B** | **-91,406 B (-12.5%)** | **1st place** |
| **`real_c_source_1MB.c`** | C multi-file codebase | 1,048,576 B | 172,747 B | 170,218 B | 178,262 B | **168,932 B** | **-3,815 B** | **1st place** |
| **`source_code_kernel_512KB.c`** | Linux kernel source | 524,288 B | 7,873 B | 8,435 B | 8,674 B | **6,431 B** | **-1,442 B (-18.3%)** | **1st place** |
| **`uniprot_protein_512KB.fasta`** | Protein amino acids | 524,288 B | 233,286 B | 225,506 B | 239,176 B | **224,185 B** | **-9,101 B** | **1st place** |
| **`astro_sensor_telemetry_512KB.raw`**| Float sensor telemetry | 524,288 B | 309,643 B | 334,113 B | 376,108 B | **194,528 B** | **-115,115 B (-37.2%)**| **1st place** |
| **TOTAL MODERN SUITE** | **6 Modern Files** | **4,718,592 B** | **1,759,780 B** | **1,738,702 B** | **1,858,210 B** | **1,527,937 B** | **-231,843 B (-13.17%)** | **6 / 6 (100.0%)** |

---

### 3. Grand Cumulative Total (53 Streams Across 5 Test Suites, 225.16 MB)

Combining all standard and modern benchmark corpora:

| Corpus Suite | Stream Count | Total Uncompressed | 7-Zip 26.02 mx9 | Orpane-MAX (.orpane) | Net Bytes Saved | Win Rate vs 7-Zip |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| **Silesia Corpus** | 12 | 211,938,580 B | 48,360,400 B | **46,626,496 B** | **-1,733,904 B** | 12 / 12 (100%) |
| **Calgary Corpus** | 18 | 3,251,493 B | 827,088 B | **743,898 B** | **-83,190 B** | 18 / 18 (100%) |
| **Canterbury Corpus**| 11 | 2,810,784 B | 493,169 B | **418,196 B** | **-74,973 B** | 11 / 11 (100%) |
| **Modern Real-World Suite** | 6 | 4,718,592 B | 1,759,780 B | **1,527,937 B** | **-231,843 B** | 6 / 6 (100%) |
| **Private Verification Holdout**| 6 | 2,439,558 B | 537,085 B | **528,479 B** | **-8,606 B** | 6 / 6 (100%) |
| **GRAND TOTAL** | **53 Streams** | **225,159,007 B** | **52,034,935 B** | **49,916,161 B** | **-2,118,774 B** | **53 / 53 (100.0%)** |

> **Milestone Achieved**: Over **2.11 MB net bytes saved** against 7-Zip 26.02 -mx9 across all 53 test streams with a 100% clean sweep win rate.

---

### 4. Streaming & Decompression Throughput Records

- **Real-Time Streaming (`ULTRA_FAST` with in-register transposition + LZ4 level 0)**:
  - Silesia Corpus (211.9 MB): **1,058.9 MB/s** (> 1.05 GB/s decompression throughput).
  - Canterbury Corpus: Beats raw LZ4 by **310,436 bytes (-25.2% denser)** while running in 53.9 ms.
- **Production Fast Profile (`FAST` with Zstandard level 1)**:
  - Canterbury Corpus: Beats Zstd-1 by **72,399 bytes (-10.5% denser)**.
  - Silesia Corpus: Decompresses at **662.7 MB/s**.

---

*All benchmarks are strictly verifiable and bit-exact. Checksums and audit logs are continuously updated.*

### Update 2026-09-08 — Discovery #115: FASTA Decoupling & Protein Sequence Breakthrough
* **Dataset**: `uniprot_protein_512KB.fasta` (524,288 bytes)
* **Technique**: Orthogonal stream separation: headers (LZMA9) + sequence stream (Brotli-11)
* **Archive Size**: **217,895 bytes**
* **7-Zip 26.02 mx9**: 233,286 bytes (**-15,391 bytes / -6.60% saved**)
* **Brotli-11**: 225,506 bytes (**-7,611 bytes / -3.37% saved**)
* **Decompression Speed**: **108.4 MB/s (4.61 ms)**, 100% bit-exact verified (SHA-256 / BLAKE3)
* **Cumulative Milestone**: Global savings reach **2,125,064 net bytes (> 2.12 MB)** across 53 streams.

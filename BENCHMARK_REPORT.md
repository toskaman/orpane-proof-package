# Scientific Benchmark Report: Orpane Physical Frontier (REAL-R2)

> 🕒 **Last Updated**: 2026-09-24 23:48:00 UTC+2 (September 24, 2026)  
> 💻 **Hardware Rig**: AMD Ryzen 7 5700X 8-Core (16 threads), 32 GB DDR4-3200 RAM, Windows 10 Pro 64-bit  
> ⏱️ **Protocol**: In-memory warmed throughput (computational execution in RAM, isolating storage I/O)  
> 🎯 **Standard Baselines**: 7-Zip 26.02 (-mx=9), Brotli 1.2.0 (-11), Zstandard 1.5.7 (-22), LZMA 5.6.3 (-9), Bzip2 1.0.8 (-9), Gzip (-9), NanoZip 0.08a (-cO -m2048m)  
> 🔬 **Independent Verifier**: Standalone native binary `bin/orpane-dec.exe` (pure Rust, LTO-stripped, 854,400 Bytes / 834.4 KiB). All files 100% bit-exact reversible.

---

```diff
+ REAL-R2 Physical Milestone:   58,505,267 B ➔ 52,303,263 B (-6,202,004 B, -10.60% net delta)
+ Physical Space Reclaimed:     -6.20 MB directly saved on canonical CANONICAL_60 benchmark suite
+ Unconstrained Probing:        Refactored boundary engine to allow dynamic probe up to 8 MB
+ Highest-Impact Stream Gains:  mozilla (-1.95MB), webster (-1.25MB), sao (-557KB), nci (-422KB)
+ Standalone Decoder Binary:    854,400 Bytes (834.4 KiB) — verified safe (+189.6 KiB below 1024 KiB ceiling)
+ Verified Invariants:          100% bit-exact reversible byte-for-byte across all 60 streams (60/60 pass)
```

---

## 🔬 Empirical Reference Matrix (CANONICAL_60 Physical Results)

| Profile / Codec | Evaluated Scope | Canonical Size | Global Ratio | Delta vs Baseline | Integrity Status |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **REAL Baseline** | `CANONICAL_60` | **58,513,656 B** | 3.858 : 1 | 0 B | 🟢 100% Bit-Exact |
| **REAL-R1** | `CANONICAL_60` | **58,505,267 B** | 3.859 : 1 | -8,389 B (-0.01%) | 🟢 100% Bit-Exact |
| **REAL-R2** | `CANONICAL_60` | 🟢 **52,303,263 B** | 🟢 **4.316 : 1** | 🟩 **-6,210,393 B (-10.61%)** | 🟢 **100% Bit-Exact** |

---

## 📁 Per-File Breakdown for High-Impact Streams in REAL-R2

| Stream | Raw Bytes | REAL Baseline (B) | REAL-R2 (B) | Physical Delta | Ratio | Selected Pipeline |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| **mozilla** | 51,220,480 | 15,823,923 | 🟢 **13,872,377** | 🟩 **-1,951,546 B** | 3.692 : 1 | Brotli-Q11 |
| **webster** | 41,458,703 | 9,678,641 | 🟢 **8,428,687** | 🟩 **-1,249,954 B** | 4.919 : 1 | Brotli-Q11 |
| **sao** | 7,251,944 | 5,143,098 | 🟢 **4,586,204** | 🟩 **-556,894 B** | 1.581 : 1 | Brotli-Q11 |
| **nci** | 33,553,445 | 1,941,829 | 🟢 **1,519,880** | 🟩 **-421,949 B** | 22.076 : 1 | Brotli-Q11 |
| **samba** | 21,606,400 | 4,162,863 | 🟢 **3,766,452** | 🟩 **-396,411 B** | 5.736 : 1 | Brotli-Q11 |
| **dickens** | 10,192,446 | 3,170,842 | 🟢 **2,827,889** | 🟩 **-342,953 B** | 3.604 : 1 | Brotli-Q11 |
| **osdb** | 10,085,684 | 3,127,422 | 🟢 **2,816,390** | 🟩 **-311,032 B** | 3.581 : 1 | Brotli-Q11 |
| **x-ray** | 8,474,240 | 4,506,231 | 🟢 **4,242,306** | 🟩 **-263,925 B** | 1.998 : 1 | Delta(2) + Q11 |
| **reymont** | 6,627,202 | 1,558,507 | 🟢 **1,332,270** | 🟩 **-226,237 B** | 4.974 : 1 | Brotli-Q11 |
| **xml** | 5,345,280 | 481,358 | 🟢 **430,507** | 🟩 **-50,851 B** | 12.416 : 1 | Brotli-Q11 |

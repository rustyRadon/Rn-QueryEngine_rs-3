# Rn-QueryEngine_rs-3

## RustyRadon-V3: High-Performance Vectorized Query Engine

**Tech Stack:** Rust, Rayon (Parallelism), Bytemuck (Zero-Copy), Memmap2 (I/O)

RustyRadon-V3 is a systems-level analytical engine designed to demonstrate "Mechanical Sympathy"—aligning software logic with modern CPU architecture to achieve sub-millisecond latency across million-row datasets.

### Key Engineering Achievements:

* **Vectorized Dispatcher:** Implemented a pattern that identifies data types once per column, eliminating per-row branch mispredictions and allowing the CPU to maintain high-speed execution loops.
* **Zero-Copy I/O:** Leveraged `mmap` to project binary files directly into memory addresses. Combined with `bytemuck` for raw byte casting, the engine achieves $O(1)$ memory overhead relative to data size.
* **Parallel Morsel Processing:** Utilizes `Rayon` to shard datasets into morsels, processing them concurrently across all CPU cores while maximizing L1/L2 cache locality.
* **Hardware-Accelerated Bitmask:** Built a custom `BitMask` utility using bitwise operations and the CPU's `POPCNT` instruction for near-instant row filtering and counting.
* **Query Profiler:** Integrated a "Filter Selectivity" tracker to visualize data reduction at each stage of the query pipeline (Dept -> Age -> Salary).

##  The "Best of the Best" Architecture

This engine is designed with **Mechanical Sympathy**—aligning software logic with how modern CPU hardware actually works.

### 1. Vectorized Dispatcher
Instead of checking data types for every single row (which causes branch misprediction), our engine uses a **Dispatcher pattern**. It identifies the column type once and "dispatches" the entire data block to a specialized math station. This allows the CPU to stay in a "tight loop."

### 2. Zero-Copy Storage (Mmap)
We use `mmap` to map binary files directly into the process address space. Data is never "loaded" or "parsed." We treat the disk as an extension of RAM, using `bytemuck` to cast raw bytes into typed slices (`&[i32]`, `&[f64]`) with **zero CPU overhead**.

### 3. Bitmask Filtering
Filters do not create new arrays. They generate a high-speed `BitMask`. Subsequent aggregation stations use this mask to selectively process data, keeping memory usage constant regardless of result size.

### 4. Data-Parallel Execution
Powered by `Rayon`, the engine automatically shards 1,000,000+ row morsels across all available CPU cores. Each core processes a contiguous chunk of memory, maximizing **L1/L2 cache hits** and triggering **SIMD (Single Instruction, Multiple Data)** hardware acceleration.

##  Folder Structure
- `src/catalog`: The Brain. Handles JSON metadata and schema evolution.
- `src/storage`: The Muscles. Manages Mmap handles and the Columnar Vault.
- `src/compute`: The Math. Contains specialized, parallelized execution stations.
- `src/util`: The Tools. High-speed bitset implementations and hardware counters.

## Performance Goal
- **Throughput:** >500 million rows/sec per core.
- **Latency:** Sub-millisecond execution for million-row aggregations.
- **Memory Footprint:** Near-zero (O(1) relative to data size due to Mmap).

## this command
to query for Department 1, employees older than 30, earning more than $50,000:
cargo run --release -- 1 30 50000

## abt
catalog/: Dynamic schema management.
storage/: High-speed, zero-copy mmap handles.
compute/: SIMD-ready parallel math stations.
util/: A hardware-accelerated bitmask.
main.rs: A coordinator with a built-in profiler.
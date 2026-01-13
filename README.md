# Rn-QueryEngine_rs-3

# RustyRadon-V3: Vectorized Columnar Query Engine

A high-performance analytical query engine built in Rust, leveraging zero-copy memory mapping and vectorized parallel execution.

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
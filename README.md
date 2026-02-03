# CHRONOS: High-Performance LSM Key-Value Store 🦀

A persistent, concurrent Key-Value store built from scratch in Rust.

## Features
- **LSM Tree Architecture** (Log-Structured Merge-Tree).
- **WAL Persistence** (Write-Ahead Log) for crash recovery.
- **Multithreading** via `RwLock` & `Arc` (Concurrent Reads).
- **Custom TCP Protocol**.

## Usage
```bash
cargo run --bin s9_dia1_velocity
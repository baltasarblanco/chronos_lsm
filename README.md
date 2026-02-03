# 🦀 CHRONOS: High-Performance LSM Key-Value Store

![Rust](https://img.shields.io/badge/rust-stable-orange?style=flat-square)
![Architecture](https://img.shields.io/badge/architecture-LSM%20Tree-blue?style=flat-square)
![Concurrency](https://img.shields.io/badge/concurrency-RwLock%20%2F%20Multithreaded-green?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-lightgrey?style=flat-square)

> *"Entropy is inevitable. Data loss is not."*

**Chronos** is a persistent, concurrent, and high-performance Key-Value store built from scratch in **Rust**. It implements a **Log-Structured Merge-Tree (LSM)** architecture similar to RocksDB or LevelDB, coupled with a custom **TCP Protocol** for remote access.

Designed as an educational deep-dive into systems engineering, Chronos bridges the gap between simple in-memory hashmaps and production-grade databases like Redis.

---

## ⚡ Key Features

### 🧠 **Symbiotic Architecture (The "Klyntar" Protocol)**
- **Hybrid Storage Engine:** Uses an in-memory `MemTable` (HashMap) for nanosecond-latency reads and disk-based `SSTables` for long-term storage.
- **Write-Ahead Log (WAL):** Guarantees **Durability (ACID)**. Every write is appended to a log file before acknowledgement. If the server crashes, Chronos replays the WAL upon restart to restore the state (0% Data Loss).

### 🚀 **High-Performance Concurrency**
- **Multithreaded Server:** Handles thousands of concurrent TCP connections using a thread-pool architecture.
- **Lock-Free Reads:** Implements `Arc<RwLock<T>>` to allow **multiple simultaneous readers** without blocking. Writers only block when absolutely necessary.
- **Zero-Copy Parsing:** Custom protocol parser minimizes memory allocations during request handling.

### 🛡️ **Self-Healing & Maintenance**
- **Crash Recovery:** Automatic "Rehydration" mechanism restores database state from disk on boot.
- **Compaction (Planned):** Background threads merge logs to prevent disk saturation and optimize read paths.

---

## 🛠️ Architecture Overview

The system is composed of three distinct layers:

1.  **The Interface (Network Layer):**
    - Raw TCP Sockets.
    - Custom Serialization Protocol (text-based for MVP, binary planned).
2.  **The Brain (Concurrency Layer):**
    - `Arc` (Atomic Reference Counting) for shared memory.
    - `RwLock` (Read-Write Lock) for thread safety.
3.  **The Core (Storage Layer):**
    - **MemTable:** Volatile RAM storage.
    - **WAL:** Append-only persistence file (`chronos.db`).

---

## 🚀 Quick Start

### Prerequisites
- Rust (latest stable)
- Netcat (`nc`) for testing (or the upcoming Chronos CLI).

### 1. Clone the Repository
```bash
git clone [https://github.com/baltasarblanco/chronos_lsm.git](https://github.com/baltasarblanco/chronos_lsm.git)
cd chronos_lsm
```

## 2. Run the Server (High Performance Mode)
```bash
cargo run --bin s9_dia1_velocity
```
You should see:
```bash
🚀 KLYNTAR v3.0 (HIGH PERFORMANCE) ACTIVE Mode: Read-Write Lock (Real Concurrency)
```

3. Connect via TCP
Open a new terminal and act as a client:
```bash
nc 127.0.0.1 8080
```

4. Issue Commands
```bash
SET user:101 {"name": "Venom", "role": "Symbiote"}
GET user:101
PING
```

## 🧪 Benchmarks & Performance

Tests performed on local environment (Dev Build).

Operation,Mechanism,Concurrency,Outcome
Write (SET),Mutex Lock (Exclusive),1 Writer,Atomic Safety
Read (GET),RwLock (Shared),Unlimited Readers,Non-Blocking
Persistence,File Append (WAL),Sync,Crash Proof

Stress Test: A "Heavy Read" (simulated 5s delay) does NOT block other clients. While one user performs a complex query, others can still ping and read instantly.

## 🗺️ Roadmap (Project AETHER)
1.  **[x] Phase 1: The Engine (LSM Tree, Memory Management)**
2.  **[x] Phase 2: The Network (TCP Server, Protocol Parser)**
3.  **[x] Phase 3: The Persistence (WAL, Crash Recovery)**
4.  **[x] Phase 4: Distributed Computing (AETHER)**
    - Implement a WebAssembly (WASM) runtime to allow users to upload custom logic.
    - Transform Chronos from a passive DB into an active Stream Processor.

## 👨‍💻 Author

Baltasar Blanco - Systems Engineer / Rustacean Building infrastructure from the atom up.
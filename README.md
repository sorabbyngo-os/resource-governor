<div align="center">
<br/>

# resource-governor

### Dynamic resource allocation and QoS enforcement across all workloads on Sorabbyngo OS.

<br/>

[![Division](https://img.shields.io/badge/Division-Sorabbyngo_OS-6C3FC4?style=for-the-badge)](https://github.com/sorabbyngo-os)
[![Lang](https://img.shields.io/badge/Lang-Rust-AA66FF?style=for-the-badge)](https://github.com/sorabbyngo-os/resource-governor)
[![Status](https://img.shields.io/badge/Status-In_Development-FF6B35?style=for-the-badge)](https://github.com/sorabbyngo-os/resource-governor)

</div>

---

## What It Does

resource-governor manages CPU, memory, GPU, I/O, and network resources across all workloads running on Sorabbyngo OS. It enforces QoS policies that prioritise critical workloads (Byngox agents, sentinel-runtime, Byngonet) over background tasks — especially important on resource-constrained edge hardware.

---

## Features

| Feature | Description |
|---|---|
| **Workload Classification** | Classifies workloads: critical, interactive, background, idle |
| **CPU Governor** | Dynamic CPU frequency and core allocation per workload class |
| **Memory Governor** | Memory pressure response — reclaim, compress, OOM priority |
| **GPU Allocation** | GPU time-slicing for Byngox model inference vs user workloads |
| **I/O QoS** | Block I/O scheduling — critical writes never starved |
| **Network QoS** | Byngonet mesh traffic prioritised over user traffic |
| **Power Management** | Battery-aware throttling for edge/IoT deployments |

---

## Architecture

```
resource-governor/
├── src/
│   ├── classification/ # Workload classification engine
│   ├── cpu/            # CPU frequency and core allocation
│   ├── memory/         # Memory pressure response and OOM policy
│   ├── gpu/            # GPU time-slicing and allocation
│   ├── io/             # Block I/O QoS via cgroups blkio
│   ├── network/        # Network traffic QoS (tc/eBPF)
│   └── power/          # Power-aware throttling
├── Cargo.toml
└── README.md
```

---

## Priority Classes

```
CRITICAL    — sentinel-runtime, Byngonet daemon, identity-core
HIGH        — Byngox agent swarm, active user session
NORMAL      — User applications, background services
LOW         — Build jobs, telemetry upload, package updates
IDLE        — Indexing, cleanup, analytics
```

---

## Stack

- **Language:** Rust
- **Kernel interface:** cgroups v2, eBPF, cpufreq, blkio
- **OS target:** All Sorabbyngo OS surfaces

---

*Sorabbyngo OS · Kisii, Kenya · 2026*

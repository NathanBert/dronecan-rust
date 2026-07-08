# dronecan-rust

**A native Rust implementation of DroneCAN** — a complete `no_std` protocol stack, compatible with the `embedded-hal` ecosystem and designed for real-time embedded systems (STM32, RP2040, ESP32, etc.).

[![Cargo Build Status](https://img.shields.io/github/actions/workflow/status/NathanBert/dronecan-rust/ci.yml?branch=main\&label=CI)](https://github.com/NathanBert/dronecan-rust/actions)
[![License](https://img.shields.io/crates/l/dronecan)](./LICENSE)
[![No\_std](https://img.shields.io/badge/no__std-compatible-green)](./README.md)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](./README.md)
[![Documentation](https://img.shields.io/docsrs/dronecan-core)](./README.md)
[![Crates.io](https://img.shields.io/crates/v/dronecan-core.svg)](./README.md)
[![Downloads](https://img.shields.io/crates/d/dronecan-core.svg)](./README.md)

---

## 📖 Overview

DroneCAN is an embedded communication protocol widely used in UAV systems such as PX4, ArduPilot, and many other autonomous platforms.

This project aims to provide a modern Rust alternative to existing implementations such as `libcanard` (C) and `canadensis` (Cyphal/Rust), featuring:

* **Memory safety** guaranteed by Rust's type system
* **High test coverage** with unit and integration tests
* **Automated DSDL code generation**
* **Full interoperability** with the existing ecosystem (PX4, ArduPilot, libcanard)

Unlike `umi-eng/dronecan-rust` (which targets `std` environments), `dronecan-rust` is designed for allocator-free microcontrollers, using exclusively `heapless` data structures and `embedded-hal` abstractions.

---

## 🏗️ Architecture

The project is organized as a multi-crate **Cargo workspace**:

| Crate                                                         | Description                                                   | Status        |
| ------------------------------------------------------------- | ------------------------------------------------------------- | ------------- |
| [`dronecan-core`](./crates/dronecan-core)                     | Core protocol implementation (CRC, fragmentation, reassembly) | ✅ In progress |
| [`dronecan-transport-can`](./crates/dronecan-transport-can)   | CAN transport abstraction (`embedded-can`)                    | 🔜 Planned    |
| [`dronecan-node`](./crates/dronecan-node)                     | High-level API (publishers, subscribers, services)            | 🔜 Planned    |
| [`dronecan-dsdl`](./crates/dronecan-dsdl)                     | DSDL parser and AST generation                                | 🔜 Planned    |
| [`dronecan-dsdl-gen`](./crates/dronecan-dsdl-gen)             | Rust code generator                                           | 🔜 Planned    |
| [`dronecan-types`](./crates/dronecan-types)                   | Pre-generated standard message types                          | 🔜 Planned    |
| [`dronecan-testkit`](./crates/dronecan-testkit)               | Virtual CAN bus for hardware-free testing                     | 🔜 Planned    |
| [`dronecan-mavlink-bridge`](./crates/dronecan-mavlink-bridge) | MAVLink ↔ DroneCAN bridge                                     | 📝 Long-term  |

---

## 🚀 Current Features

### `dronecan-core` (WIP)

* ✅ Encode/decode 29-bit CAN identifiers (priority, message type, node ID)
* ✅ Multi-frame transfer handling (Start, Middle, End, Single)
* ✅ Correct CRC-16/CCITT-FALSE implementation (`poly=0x1021`, `init=0xFFFF`)
* ✅ Payload handling (internal CRC, tail byte)
* ✅ Unit tests using golden frames from `libcanard` and `pydronecan`

---

## Example

```rust
use dronecan_core::DroneCanFrame;
use embedded_can::Frame;

// Create a frame from an extended CAN ID and payload
let frame = DroneCanFrame::new(
    ExtendedId::new(0x18FF0001),
    &[0x01, 0x02, 0x03],
);

// Parse an incoming frame
if let Some(rx_frame) = DroneCanFrame::new(id, data) {
    println!("Type: {:?}", rx_frame.mtid);
}
```

---

## 🛠️ Build & Usage

### Build

```bash
# Build the complete workspace
cargo build --workspace
```

### Tests

```bash
# Run all workspace tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p dronecan-core
```

---

## 🔬 Testing Methodology

Each crate contains internal `#[cfg(test)]` modules validating:

* CRC computation
* CAN ID encoding/decoding
* Buffer handling
* Payload serialization/deserialization
* Error handling

---

## 🎯 Roadmap

### Phase 1 — Protocol Core

* [ ] DroneCAN transfer handling
* [ ] CRC encoding/decoding
* [ ] Multi-frame fragmentation and reassembly
* [ ] Unit test coverage

### Phase 2 — Functional Node

* [ ] Hardware CAN interfaces (STM32 FDCAN, ESP32 TWAI)
* [ ] Node publishers/subscribers
* [ ] Service handling
* [ ] Dynamic node allocation

### Phase 3 — DSDL Ecosystem

* [ ] DSDL parser
* [ ] Rust code generator
* [ ] Pre-generated standard message types

### Phase 4 — Production Readiness

* [ ] Validated PX4 and ArduPilot compatibility
* [ ] Complete documentation
* [ ] Hardware examples

---

## 📚 Documentation

* [DroneCAN Specification](https://dronecan.github.io/Specification/)
* [Official Implementations](https://dronecan.github.io/Implementations/)

---

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch:

```bash
git checkout -b feature/my-new-feature
```

3. Commit your changes:

```bash
git commit -am "Add new feature"
```

4. Push your branch:

```bash
git push origin feature/my-new-feature
```

5. Open a Pull Request

---

## 📄 License

Dual-licensed under your choice of:

* MIT License
* Apache License 2.0

---

## 📞 Support

* GitHub Issues: https://github.com/NathanBert/dronecan-rust/issues

---

## 🙏 Credits

Inspired by:

* [`libcanard`](https://github.com/dronecan/libcanard) (C implementation)
* [`canadensis`](https://github.com/samcrow/canadensis) (Rust Cyphal implementation)
* [`umi-eng/dronecan-rust`](https://github.com/umi-eng/dronecan-rust) (Rust DroneCAN implementation)

---

**Current status:** Alpha — API unstable, active development, test coverage improving.

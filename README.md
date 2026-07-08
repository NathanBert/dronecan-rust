# dronecan-rust

**Implémentation native Rust de DroneCAN** — stack protocolaire complète, `no_std`, compatible avec l'écosystème `embedded-hal` et pensée pour les systèmes embarqués temps réel (STM32, RP2040, ESP32, etc.).

[![Cargo Build Status](https://img.shields.io/github/actions/workflow/status/NathanBert/dronecan-rust/ci.yml?branch=main&label=CI)](https://github.com/NathanBert/dronecan-rust/actions)
[![License](https://img.shields.io/crates/l/dronecan)](./LICENSE)
[![No_std](https://img.shields.io/badge/no__std-compatible-green)](./README.md)

---

## 📖 Vue d'ensemble

DroneCAN est le protocole de communication embarqué utilisé par PX4, ArduPilot et de nombreux systèmes UAV. Ce projet vise à fournir une alternative Rust crédible à `libcanard` (C) et `canadensis` (Cyphal/Rust), avec :

- **Sécurité mémoire** garantie par le typage Rust
- **Tests unitaires/intégration** avec couverture élevée
- **Génération de code DSDL** automatisée
- **Interopérabilité complète** avec l'écosystème existant (PX4, ArduPilot, libcanard)

Contrairement à `umi-eng/dronecan-rust` (qui cible `std`), `dronecan-rust` est conçu pour fonctionner sur des microcontrôleurs sans allocateurs, en utilisant exclusivement `heapless` et `embedded-hal`.

---

## 🏗️ Architecture

Le projet est organisé en **workspace Cargo** multi-crates :

| Crate | Rôle | Statut |
|-------|------|--------|
| [`dronecan-core`](./crates/dronecan-core) | Protocole pur (CRC, fragmentation, réassemblage) | ✅ en cours |
| [`dronecan-transport-can`](./crates/dronecan-transport-can) | Couche d'abstraction CAN (`embedded-can`) | 🔜 à venir |
| [`dronecan-node`](./crates/dronecan-node) | API haut niveau (publishers, subscribers, services) | 🔜 à venir |
| [`dronecan-dsdl`](./crates/dronecan-dsdl) | Parseur DSDL vers AST | 🔜 à venir |
| [`dronecan-dsdl-gen`](./crates/dronecan-dsdl-gen) | Générateur de code Rust | 🔜 à venir |
| [`dronecan-types`](./crates/dronecan-types) | Messages standards pré-générés | 🔜 à venir |
| [`dronecan-testkit`](./crates/dronecan-testkit) | Bus CAN virtuel pour tests sans hardware | 🔜 à venir |
| [`dronecan-mavlink-bridge`](./cronecan-mavlink-bridge) | Pont MAVLink ↔ DroneCAN | 📝 à long terme |

---

## 🚀 Fonctionnalités actuelles

### `dronecan-core` (WIP)

- ✅ Encodage/décodage des IDs CAN 29 bits (priorité, type de message, node ID)
- ✅ Gestion des transfers multi-frame (Start, Middle, End, Single)
- ✅ Calcul CRC-16/CCITT-FALSE correct (polynôme `0x1021`, init `0xFFFF`)
- ✅ Structuration des payloads (CRC interne, tailbyte)
- ✅ Tests unitaires avec Golden Frames (libcanard, pydronecan)

### Exemple rapide

```rust
use dronecan_core::DroneCanFrame;
use embedded_can::Frame;

// Créer une frame depuis un ID étendu et un payload
let frame = DroneCanFrame::new(ExtendedId::new(0x18FF0001), &[0x01, 0x02, 0x03]);

// Parser une frame reçue
if let Some(rx_frame) = DroneCanFrame::new(id, data) {
    println!("Type: {:?}", rx_frame.mtid);
}
```

---

## 🛠️ Compilation et usage

### Build

```bash
# Build complet workspace
cargo build --workspace

```

### Tests

```bash
# Tous les tests workspace
cargo test --workspace

# Tests unitaires une crate
cargo test -p dronecan-core
```

---

## 🔬 Méthodologie de tests

### Tests unitaires

Chaque crate possède un module `#[cfg(test)]` interne pour valider :

- CRC, buffers, IDs CAN
- Encodage/décodage des payloads
- Gestion des erreurs

---

## 🎯 Roadmap

### Phase 1 — Core protocolaire

- [ ] Gestion des transfers DroneCAN
- [ ] Encodage/décodage CRC
- [ ] Fragmentation multi-frame
- [ ] Tests unitaires

### Phase 2 — Node fonctionnel

- [ ] Interface CAN hardware (STM32 FDCAN, ESP32 TWAI)
- [ ] Node avec publishers/subscribers
- [ ] Gestion des services
- [ ] Node allocation

### Phase 3 — DSDL

- [ ] Parseur DSDL
- [ ] Générateur Rust
- [ ] Messages standards pré-générés

### Phase 4 — Industrialisation (🔜 0%)

- [ ] Compatibilité PX4/ArduPilot validée
- [ ] Documentation complète
- [ ] Exemples hardware

---

## 📚 Documentation

- [Spec DroneCAN](https://dronecan.github.io/Specification/)
- [Implementations officielles](https://dronecan.github.io/Implementations/)

---

## 🤝 Contribuer

1. Forke le repo
2. Crée ta feature branch (`git checkout -b feature/ma-nouvelle-fonction`)
3. Commit (`git commit -am 'Ajoute une fonction'`)
4. Push (`git push origin feature/ma-nouvelle-fonction`)
5. Ouvre une Pull Request

---

## 📄 Licence

Dual-licensed sous **MIT** ou **Apache-2.0** à votre choix.

---

## 📞 Support

- Issues : [GitHub Issues](https://github.com/NathanBert/dronecan-rust/issues)

---

## 🙏 Crédits

Inspiration directe de :

- [`libcanard`](https://github.com/dronecan/libcanard) (C)
- [`canadensis`](https://github.com/samcrow/canadensis) (Rust Cyphal)
- [`umi-eng/dronecan-rust`](https://github.com/umi-eng/dronecan-rust) (Rust DroneCAN)

---

**Statut actuel** : Alpha, API non stable, tests en cours.

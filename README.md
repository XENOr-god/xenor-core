# XENØr Core

Deterministic graph propagation and incentive routing engine written in Rust.

XENØr Core is the foundational engine for modeling and executing verifiable incentive flows in protocol-native systems.

The system focuses on deterministic reward propagation, auditable incentive structures, and simulation-first development.

---

## Why this exists

Most token systems rely on opaque incentive logic.

XENØr explores a different approach:

- deterministic propagation
- verifiable reward routing
- protocol-native incentive structures
- simulation-driven design

The goal is to design systems where incentive movement can be inspected, tested, and reproduced.

---

## Architecture

XENØr Core models token flows as a deterministic graph propagation system.

Nodes represent participants or protocol components.

Edges represent incentive relationships.

Reward propagation follows deterministic routing rules which makes the system:

- reproducible
- auditable
- simulation-friendly

---

## Repository structure

## Relationship to other repositories

xenor-core  
Reusable protocol primitives and deterministic propagation engine.

xenor-sim  
Simulation environment used to test system behavior.

xenor-sale  
Archived experiment of an early bonding-curve prototype.

---

## Status

Early research and development.

No token launch.

Open build.

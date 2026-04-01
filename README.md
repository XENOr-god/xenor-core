# XENØr Core

Deterministic graph propagation and conservative reward-routing engine written in Rust.

`xenor-core` is the execution layer in the XENØr stack. It keeps propagation rules inspectable, reproducible, and small enough to reason about directly.

## Focus

- deterministic propagation
- conservative reward routing
- auditable state movement
- simulation-ready primitives

## Design Principles

- identical inputs should produce identical outputs
- routing rules should remain explicit and bounded
- the core should stay small enough to inspect directly
- simulation and public communication stay in separate repositories

## Related Repositories

- [`xenor-sim`](https://github.com/XENOr-god/xenor-sim) — scenario testing and experiment runner
- [`xenor-site`](https://github.com/XENOr-god/xenor-site) — canonical public surface for architecture and repository status
- [`xenor-sale`](https://github.com/XENOr-god/xenor-sale) — archived sale prototype kept only as historical research context

## Local Development

```bash
cargo check
cargo test
cargo run
```

Architecture notes live in `docs/architecture.md`.

## Status

Early research engine. No token launch logic lives here. Interfaces may still change as the stack evolves.

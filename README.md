# xenor-core

`xenor-core` is the deterministic execution and protocol-systems layer in the
XENOr stack. It keeps graph propagation, routing logic, and state transitions
small enough to inspect directly and stable enough to serve as the base for
simulation and public communication.

## Status

Active, research-stage core repository. Public release tags exist, but the core
interfaces and surrounding stack are still evolving and should not be read as a
stable public API surface yet.

## Why This Repo Exists

This repository exists to isolate deterministic protocol logic from the rest of
the stack. `xenor-core` should hold the reusable execution rules, not the
scenario harness, not the public website, and not launch-specific experiments.

## Relationship to the XENOr Stack

- `xenor-core` is the deterministic execution/core systems layer
- `xenor-sim` builds scenarios and validation runs on top of `xenor-core`
- `xenor-site` is the canonical public surface and repository map
- `xenor-engine` is a lower-level deterministic engine substrate, not the main
  XENOr protocol logic layer

## Quick Start / Local Development

Toolchain: stable Rust with edition 2024 support.

```bash
cargo check
cargo test
cargo run
```

- `cargo run` executes the current sample entry point in
  [`src/main.rs`](src/main.rs).
- Architecture notes live in [`docs/architecture.md`](docs/architecture.md).

## Repository Boundaries / Non-goals

- This is not the canonical public website. Use `xenor-site` for that.
- This is not the main scenario harness. Use `xenor-sim` for validation runs.
- This is not a launch or sale repository.
- The core should stay deterministic and readable before it grows broader.

## Related Repositories

- [`xenor-site`](https://github.com/XENOr-god/xenor-site) — canonical public
  surface and first stop for newcomers
- [`xenor-sim`](https://github.com/XENOr-god/xenor-sim) — scenario and
  validation layer built around `xenor-core`
- [`xenor-engine`](https://github.com/XENOr-god/xenor-engine) — deterministic
  engine and replay/snapshot substrate
- [`xenor-sale`](https://github.com/XENOr-god/xenor-sale) — archived
  historical sale prototype

## Contributing

No standalone contributing guide is currently published in this repository.
Use issues or pull requests directly for targeted core fixes.

## License

Released under the [MIT License](LICENSE).

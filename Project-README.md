# SiYuan Project Exploration Overlay Generator

This is the governance guide for the Rust implementation. For public usage,
start with `README.md`.

## Read First

1. `SiYuan-Overlay-Generator.manifest.toml`
2. `docs/Project-Proposal.md`
3. `docs/Scope-Boundary.md`
4. `docs/Success-Criteria.md`
5. `docs/Risk-Assessment.md`
6. `docs/Architecture.md`
7. `docs/Discovery-Contract.md`
8. `docs/Import-Validation-Plan.md`
9. `docs/Package-Layout.md`
10. `docs/CLI-Contract.md`
11. `docs/Roadmap.md`

## Governance

- **PPS** — project mission, boundaries, success/failure criteria, constraints, risks, roadmap.
- **WGS** — project registration, placement, lifecycle, and agent orientation.
- **CTS** — v0.1 CLI command behavior and release-side automation contract.

## Current State

- WGS lifecycle: `implementation`
- PPS readiness: `ready`
- Target version: `0.1.0`
- Delivery surface: experimental Rust CLI (`siyuan-overlay-generator` package)
- Validation: representative Markdown ZIP imported successfully with SiYuan
  v3.8.3; see `docs/Import-Validation-Plan.md`.
- Native SiYuan structures: deferred

The generated SiYuan material is a reference snapshot. It is not the authoritative source tree.

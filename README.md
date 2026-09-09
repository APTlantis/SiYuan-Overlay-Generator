# SiYuan Project Exploration Overlay Generator

An experimental, local Rust CLI that turns a source directory into an
inspectable Markdown ZIP for SiYuan exploration. It creates a derived reference
snapshot; the source directory remains authoritative and is never modified.

## What it does today

- Deterministically discovers a directory tree without following symlinks.
- Builds a normalized project model before any output projection.
- Preserves Markdown as Markdown; wraps readable source and text in code blocks.
- Uses metadata-only pages for binary, unsupported, or oversized files.
- Generates an overview, indexes, directory pages, provenance, and structure map.
- Writes an inspectable Markdown ZIP for SiYuan's `Data migration` →
  `Markdown .zip` workflow.

The import path was validated with SiYuan v3.8.3 using this repository's
representative fixture. SiYuan converts resolved Markdown links into its own
internal links during import; this tool does not read or write those internals.

## Quick start

Requires a current Rust toolchain.

```powershell
cargo run -- inspect "D:\path\to\project"
cargo run -- generate "D:\path\to\project" "D:\output\project-overlay.zip"
```

The output ZIP path must be new; generation refuses to overwrite it. Inspect
the ZIP before importing it into SiYuan.

## Import notes

- Import through SiYuan's Markdown ZIP migration flow, not by copying into a vault.
- A source directory named `assets` becomes `_assets` only in the ZIP layout,
  because SiYuan treats `assets` specially during import. Visible source paths
  remain unchanged.
- Generated material may copy source content. Do not run it on a tree
  containing secrets until exclusion controls are implemented.

## Current limits

This is a validated vertical slice, not a release. It has no exclusion flags,
configurable scan limits, Git provenance, refresh workflow, or final CTS
machine-readable contract. Native SiYuan storage, direct vault mutation,
synchronization, AI generation, and cross-project intelligence are out of scope.

## Development

```powershell
cargo fmt --check
cargo test
cargo clippy -- -D warnings
```

See [Project Proposal](docs/Project-Proposal.md),
[package layout](docs/Package-Layout.md), and
[import validation](docs/Import-Validation-Plan.md).

## License

No license has been granted yet. The package is marked `UNLICENSED` and is not
published to crates.io.

# AGENTS

## Project Orientation

Read the entity-named project manifest and `docs/Project-Proposal.md` before broad project work.

The v0.1 product is a CLI that generates portable SiYuan import material from a read-only source project or directory tree.

Do not silently expand v0.1 into:
- native SiYuan Attribute Views or storage internals;
- direct live SiYuan workspace mutation;
- synchronization;
- an IDE or Git client;
- AI-required generation;
- cross-project intelligence.

The normalized project model is an architectural boundary, not an optional refinement.

When evaluating completion, judge v0.1 against `docs/Success-Criteria.md`. Future improvements that do not block the declared target are evolution findings, not completion deficits.

## Rust Baseline

- This is a Rust CLI package: use `cargo fmt`, `cargo test`, and `cargo clippy -- -D warnings` for proportionate verification when implementation changes.
- Keep the executable entry point thin. Discovery constructs the normalized model; projection consumes it to create the portable package.
- Treat the input path as read-only. Tests proving this property belong with discovery and end-to-end generation once those layers exist.
- Do not represent an unknown file as readable content. Use a metadata-only projection until a safe, explicit classification rule supports a richer representation.
- Keep generated output inspectable before import. Do not couple v0.1 to SiYuan internal storage formats or a live workspace.

## Current Bootstrap State

The package currently establishes the Cargo baseline and structural model only.
The CLI must say generation is unimplemented until discovery and projection are
actually present; do not advertise a supported generation workflow prematurely.

# Architecture — SiYuan Project Exploration Overlay Generator

## Bootstrap Decision

The implementation language is Rust. The initial Cargo package is named
`siyuan-overlay-generator`; it is local-only and has no runtime dependencies.

The executable provides help, version behavior, and a read-only structural
`inspect` command. It does not generate or write to SiYuan, and does not claim
that a portable package can yet be produced.

## Architectural Spine

```text
read-only source tree
        |
        v
discovery -----------------> normalized ProjectModel
                                      |
                                      v
                               classification
                                      |
                                      v
                          portable package projection
```

`ProjectModel` is the required seam between discovery and projection. It holds
source-derived structural facts only; later stages can add classifications and
generated presentation without making generated material authoritative.

## Initial Module Boundary

- `src/model.rs` owns the normalized structural model.
- `src/discovery.rs` reads the source tree and constructs that model.
- A future projection module may consume the model to create an inspectable
  output package.
- `src/main.rs` owns only the human-facing command entry point.

Discovery must not create output while it traverses. Projection must not rescan
or mutate the source tree. Direct SiYuan workspace access remains out of scope.

## Current Implementation Evidence

`tests/fixtures/representative-project` represents a minimal mixed project:
documentation, source, configuration, tests, and opaque artifacts. Its
discovery test verifies deterministic paths and fingerprints every fixture file
before and after discovery to prove the scan does not modify it.

## Next Implementation Evidence

Before selecting an import-package layout, validate a small representative
fixture through SiYuan's supported portable import path. The result should guide
the CLI contract, output layout, and first projection tests.

The repeatable procedure and pass condition are in
`docs/Import-Validation-Plan.md`. That environment-dependent check is pending;
no implementation currently claims portable-package compatibility.

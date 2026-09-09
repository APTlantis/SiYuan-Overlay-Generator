# Scope Boundary — SiYuan Project Exploration Overlay Generator

## Current Target

The current target is v0.1.0: a complete portable structural overlay generator delivered as a CLI.

## In Scope

- Read-only directory/project discovery.
- Normalized internal project model.
- Project overview generation.
- Portable project and directory indexes.
- Readable Markdown and source-code representations.
- Metadata-only records for binaries and unsupported artifacts.
- Snapshot/provenance metadata.
- At least one project-structure visualization.
- SiYuan-ready portable import package.
- CTS-governed CLI behavior.

## Out of Scope for v0.1.0

- Native SiYuan Attribute Views/databases.
- Direct SiYuan workspace mutation.
- Native SiYuan identifiers/storage structures.
- Incremental synchronization or refresh.
- Editing/building/testing/source-control operations.
- AI-dependent generation.
- Cross-project intelligence.
- Deep binary inspection.
- Git history visualization.

## Boundary Rule

The generated material is a derived reference snapshot. The source filesystem, Git repository, and project documentation remain authoritative.

Any feature that causes the generated SiYuan representation to become the canonical working state requires a proposal revision.

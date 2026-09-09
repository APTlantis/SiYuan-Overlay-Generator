# Roadmap — SiYuan Project Exploration Overlay Generator

## Phase 1 — Data Spine

Define the CLI boundary, normalized project model, discovery rules, generated resource categories, and provenance model. Prove them against representative fixtures.

**Current state:** substantially implemented. The Rust model, deterministic
read-only discovery, classification baseline, fixture, and immutability tests
are in place. Exclusions, configurable limits, and richer provenance remain.

## Phase 2 — Core Workflows

Implement discovery, project overview, directory surfaces, portable indexes, Markdown/source wrappers, artifact metadata, structural visualization, and package assembly.

**Current state:** implemented as an experimental vertical slice. It writes an
inspectable Markdown ZIP; SiYuan v3.8.3 import rendered the overview, indexes,
directory surfaces, code wrappers, artifact records, and structure map.

## Phase 3 — Verification

Verify source immutability, classification fallbacks, package import, project/directory/file exploration, and CTS-visible command behavior.

**Current state:** fixture source immutability and ZIP contents are automated;
the representative package was manually imported and inspected. Broader mixed
tree, error-path, size-limit, and CTS contract verification remain.

## Phase 4 — Release Readiness

Finalize the CTS command contract, help/version output, exit behavior, examples, release checklist, and v0.1.0 completion evidence.

**Current state:** not started. The current CLI contract is explicitly
provisional and this project is not release-ready.

## Deferred Evolution

Native SiYuan structures, direct workspace writing, refresh/incremental regeneration, AI enrichment, cross-project indexes, dependency intelligence, and richer Git/binary views remain outside v0.1.0.

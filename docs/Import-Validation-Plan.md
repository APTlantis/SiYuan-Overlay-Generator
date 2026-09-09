# Portable Import Validation Plan — v0.1

## Status

**Pending a local SiYuan environment.** No v0.1 package layout or import claim
is approved by this plan alone.

## Fixture

Use `tests/fixtures/representative-project`, which contains documentation,
Rust source, configuration, tests, and opaque artifacts. Later package tests
must generate a fresh output directory outside this fixture and compare the
fixture before and after generation.

## Validation Procedure

1. Generate a portable package from the fixture using the future `generate`
   command.
2. Inspect the package on disk before import. Confirm it contains an overview,
   directory landing pages, indexes, provenance, source/document wrappers,
   artifact records, and structure visualization.
3. Import it using SiYuan's ordinary supported portable import workflow, never
   through native storage manipulation.
4. In SiYuan, open the root overview and follow links to a directory, Markdown
   document, source wrapper, and artifact record.
5. Confirm provenance visibly identifies the fixture path and generation time,
   and labels the result as derived reference material.
6. Confirm no fixture file or directory changed by comparing the pre-generation
   fingerprint with the post-generation fingerprint.
7. Record SiYuan version, import workflow used, package layout revision, and
   any structural loss or navigation failures as release evidence.

## Pass Condition

The import is a pass only when the package is usable for orientation and
navigation at the project, directory, and resource levels without native
SiYuan structures. A successful file transfer alone is not sufficient.

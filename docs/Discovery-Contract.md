# Discovery Contract — v0.1 Data Spine

## Current Command

`siyuan-overlay-generator inspect <SOURCE>` performs the first implemented
Phase 1 operation. It accepts a source **directory** and reports the resulting
normalized structural model summary. This provisional command is for discovery
validation; the release CTS command surface remains to be finalized.

## Guarantees

- The source root is canonicalized for the model record.
- Every discovered entry is recorded with a path relative to that root and one
  structural kind: directory, file, symlink, or other.
- Child paths are ordered deterministically by relative path.
- Symlinks are recorded but never followed.
- The operation creates no output package and performs no writes in the source
  tree.
- An unreadable root, unreadable child, or non-directory input fails with the
  relevant path in the diagnostic.

## Deliberately Not Yet Decided

- Exclusion defaults and configurable size/file-count limits.
- Content classification and encoding policy.
- Portable SiYuan import-package layout.
- Final CTS command and machine-readable-output contract.

Those decisions follow fixture-based import validation. Until then, `inspect`
only establishes trustworthy source-derived structure.

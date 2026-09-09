# Portable Package Layout — v0.1

The current generator writes an uncompressed, standard Markdown ZIP for
SiYuan's `Data migration` → `Markdown .zip` import path. The ZIP is inspectable
with any normal archive tool before import.

Each package has one sanitized root directory containing:

- `README.md` — source-authority notice, provenance, counts, and entry links.
- `Project Index.md` — every source file and its safe generated representation.
- `Directory Index.md` and `Directories/.../README.md` — directory orientation
  and child navigation.
- `Structure.md` — a plain-text tree visualization.
- `Documents/...` — preserved Markdown documents.
- `Resources/...` — fenced source or readable-text wrappers.
- `Artifacts/...` — metadata-only records for binary, unsupported, or oversized
  files.

The output path must not already exist. This avoids silent replacement of a
previous snapshot. The generator writes the ZIP only after discovery has built
the normalized model.

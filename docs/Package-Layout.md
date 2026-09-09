# Portable Package Layout — v0.1

The current generator writes an uncompressed, standard Markdown ZIP for
SiYuan's `Data migration` → `Markdown .zip` import path. The ZIP is inspectable
with any normal archive tool before import.

The ZIP has its generated documents at its top level. SiYuan's Markdown ZIP
import supplies the project-level folder, avoiding a redundant nested root.
The top level contains:

- `Project Overview.md` — source-authority notice, provenance, counts, and
  entry links.
- `Project Index.md` — every source file and its safe generated representation.
- `Directory Index.md` and `Directories/.../Directory.md` — directory orientation
  and child navigation.
- `Structure.md` — a plain-text tree visualization.
- `Documents/...` — preserved Markdown documents.
- `Resources/...` — fenced source or readable-text wrappers.
- `Artifacts/...` — metadata-only records for binary, unsupported, or oversized
  files.

SiYuan's Markdown ZIP importer treats a directory component named `assets` as
special and does not import it as a document branch. The generator therefore
maps that component to `_assets` only in generated package paths. All visible
source-path labels and metadata continue to say `assets`.

The output path must not already exist. This avoids silent replacement of a
previous snapshot. The generator writes the ZIP only after discovery has built
the normalized model.

# Project Proposal — SiYuan Project Exploration Overlay Generator

## Project Identity

**Project Name:** SiYuan Project Exploration Overlay Generator  
**Project Class:** Command tool  
**WGS Lifecycle State:** Planning  
**PPS Readiness:** Ready  
**Target Version:** v0.1.0  
**Primary Governing Standard:** PPS  
**Delivery Standard:** CTS  
**Workspace Governance:** WGS  

**Project Theme:** Turn existing software projects and directory trees into read-oriented SiYuan exploration snapshots without displacing the original project as the authoritative source.

## Problem Statement

Software projects already contain substantial structure and context in their filesystems, source trees, documentation, manifests, tests, and artifacts, but importing that hierarchy into SiYuan does not by itself produce a useful exploration surface.

The user who experiences this problem is primarily a developer, researcher, archivist, power user, or agent trying to understand an unfamiliar, old, or partially forgotten project. Today that user can inspect the filesystem, Git repository, IDE, and individual documents directly, or import files into SiYuan and manually create the missing orientation layer.

That is insufficient because a raw hierarchy answers where files are but does not reliably explain what the project is, how directories and files should be interpreted, which resources matter, what kind of artifact each file represents, or where exploration should continue next.

## Mission Statement

SiYuan Project Exploration Overlay Generator is a local command-line tool that scans an existing software project or arbitrary directory tree, builds a normalized internal project model, and generates a SiYuan-ready reference snapshot containing structured project orientation, directory context, readable file representations, artifact metadata, indexes, provenance, and structural visualization while leaving the original filesystem, Git repository, documentation, and development environment authoritative and untouched.

## Design Principle

> The filesystem tells you where things are.  
> The overlay should help tell you what they are, how they relate, and where to look next.

## Operational Personas

- **Developer:** Uses the generated snapshot to reacquaint themselves with a project without opening every file individually.
- **Researcher / Archivist:** Uses the snapshot as a structured reference representation of a project at a known point in time.
- **Power User / Operator:** Runs the CLI against arbitrary directories and imports the result into SiYuan.
- **Agent:** Reads the generated material as structured local context for project understanding and semantic retrieval.

These are operational roles, not market personas.

## Design Boundaries

### In Scope for v0.1.0

v0.1.0 is a complete structural vertical slice. It must:

1. Accept an arbitrary project or directory tree as input.
2. Inspect the tree without modifying the source.
3. Preserve the source hierarchy in the generated representation where practical.
4. Build a normalized internal project model before projection.
5. Generate a useful root Project Overview.
6. Generate project-wide resource indexes using portable, non-native documents such as Markdown tables or equivalent import-safe structures.
7. Generate useful directory landing pages and child indexes.
8. Preserve Markdown documents as readable Markdown.
9. Represent supported source code as fenced-code Markdown documents or equivalent portable text wrappers.
10. Generate metadata pages for binaries and other unsuitable-for-inline-reading artifacts.
11. Record snapshot and provenance information sufficient to distinguish the generated material from the authoritative source.
12. Produce at least one useful project-structure visualization in an import-safe representation.
13. Package the output for safe SiYuan import.
14. Leave the original project completely untouched.

### Explicitly Deferred

The following are valid future capabilities but are not required for v0.1.0 completion:

- SiYuan-native Attribute Views / databases.
- Direct live SiYuan notebook or workspace writing.
- SiYuan-native identifiers or storage-level structures.
- Snapshot refresh and incremental regeneration.
- Change detection.
- Stable generated IDs across regenerations.
- User-annotation preservation.
- Multi-project indexes.
- Cross-project relationships.
- AI summaries.
- AI-generated architecture maps.
- Dependency extraction beyond lightweight metadata detection needed for orientation.
- Rich source/test relationship inference.
- Binary inspection beyond basic artifact metadata.
- Git history views.
- Automatic embedding controls.
- Generated stale-page handling.

### Permanent or Proposal-Revising Non-Goals

The project does not, without an explicit future proposal revision:

- Replace the filesystem.
- Replace Git or become a source-control system.
- Replace an IDE.
- Become the authoritative working copy of a project.
- Edit source code.
- Build or test the source project.
- Require real-time synchronization.
- Require cloud services.
- Require AI for its core function.
- Mutate the source project during ordinary generation.
- Treat generated material as canonical project truth.

## Success Criteria

v0.1.0 is successful when all of the following are demonstrably true:

1. A user can run the CLI against an arbitrary, representative software project or directory tree and obtain a complete generated package without modifying the input tree.
2. The package can be imported into SiYuan using the intended portable import path.
3. Opening the generated project root gives useful orientation about what the project is, its snapshot provenance, its structure, and where to explore next.
4. The generated representation provides useful navigation at project, directory, and file/artifact levels.
5. Supported text and source files are readable in SiYuan without requiring the original development environment.
6. Binaries and other non-readable resources are represented by useful metadata rather than dumped as meaningless content.
7. The user can locate resources by category or metadata through generated indexes even though native SiYuan Attribute Views are deferred.
8. At least one structural visualization materially improves understanding of the project layout.
9. Provenance makes it clear that the generated material is a reference snapshot and identifies its source location and generation time; Git metadata is included when available.
10. An unfamiliar or half-forgotten representative repository can be understood materially faster from the generated overlay than from a plain imported filesystem tree alone.

## Failure Criteria

The project has failed or requires redesign if:

- Its core workflow requires cloud services or an account.
- Its core function requires AI or an LLM.
- Generating an overlay mutates the authoritative source project.
- The generated snapshot is presented as authoritative project state rather than a derived reference.
- The generator becomes tightly coupled to filesystem traversal such that there is no normalized intermediate project model.
- v0.1.0 requires direct manipulation of SiYuan's internal/native storage representation.
- Typical generated output is less useful to explore than a straightforward directory import.
- The tool cannot represent unsupported or binary resources safely without attempting inappropriate content conversion.

## Technical Direction

### Delivery Shape

v0.1.0 is a **command-line tool** governed by CTS.

The stable command surface, flags, stdout/stderr behavior, machine-readable output, exit codes, examples, and release checklist are CTS concerns and should be defined during implementation/release preparation.

### Architecture Direction

The architectural spine is:

```text
Input Project / Directory
        |
        v
   Discovery Pass
        |
        v
Normalized Project Model
        |
        +----------------------+----------------------+-------------------+
        |                      |                      |                   |
        v                      v                      v                   v
Readable Documents       Source Wrappers       Artifact Records      Metadata
        |                      |                      |                   |
        +----------------------+----------------------+-------------------+
                               |
                               v
                      Directory Surfaces
                               |
                               v
                       Project-Level Views
                               |
                               v
                    Structure Visualization
                               |
                               v
                    SiYuan-Ready Portable Package
```

The discovery layer and SiYuan projection layer must remain conceptually separate.

### SiYuan Integration Direction for v0.1.0

v0.1.0 targets a **portable import package**, not SiYuan-native storage structures.

Project and directory "databases" from the original concept are represented for this release as portable resource indexes—primarily Markdown or another import-safe representation. Native Attribute Views, native identifiers, and direct workspace writing are deferred.

### Technology Choices

Implementation language, libraries, packaging method, and exact import-package mechanics remain open until implementation discovery. They are not required to define the project mission or completion boundary.

## Constraints

- Must operate locally.
- Must not require cloud services or accounts for the core workflow.
- Must not require AI for generation.
- Must treat the input tree as read-only during ordinary operation.
- Must preserve clear provenance between source material and generated representation.
- Must tolerate arbitrary directory trees rather than assuming one repository layout.
- Must avoid presenting unsupported classification or inferred relationships as certain facts.
- Must keep discovery/modeling separate from SiYuan projection.
- Must produce output that can be inspected before import.
- Must remain understandable and maintainable by a single maintainer.
- v0.1.0 must not depend on private or unstable SiYuan-native storage internals.

## Risk Assessment

| Risk | Type | Effect | v0.1 Control / Disposition |
| --- | --- | --- | --- |
| SiYuan import behavior differs from assumed Markdown/package semantics | Dependency / compatibility | Generated output may import poorly or lose structure | Validate the intended portable import path early with a representative fixture; native structures remain deferred. |
| Arbitrary repositories contain huge, generated, binary, or pathological trees | Technical / data | Slow scans, excessive output, unusable packages | Classification and safe representation rules must distinguish readable text from metadata-only artifacts; exact thresholds may be implementation-configurable. |
| Generated representations imply more certainty than inspection justifies | Trust | User or agent may mistake inference for source truth | Preserve provenance and label generated/inferred metadata distinctly from source facts. |
| Tight coupling between recursive traversal and document generation | Architecture | Hard-to-test code and blocked future exporters | Normalized internal project model is an architectural requirement. |
| Scope expands toward IDE, Git client, synchronization, or live SiYuan integration | Project | v0.1 never stabilizes | Explicit non-goals and deferred list govern completion. |
| Portable indexes are less powerful than native Attribute Views | Product / usability | Some filtering and navigation capability is weaker in v0.1 | Accepted for v0.1; judge the release on structural usefulness, not native integration. |
| Source content contains secrets or sensitive material | Security / trust | Generated snapshot may duplicate sensitive content into another system | The tool should not claim to sanitize arbitrary projects; documentation must make copying behavior inspectable and support exclusions before release if required for safe ordinary use. |
| Unsupported encodings or file formats produce broken wrappers | Technical | Incomplete or misleading representation | Preserve unsupported artifacts as metadata-only records instead of forcing conversion. |

## v0.1.0 Completion Boundary

### Required

- Read-only scan of arbitrary project/directory input.
- Normalized internal project model.
- Root project overview.
- Portable project resource index.
- Directory landing pages and portable child indexes.
- Readable Markdown handling.
- Source-code wrapper handling for supported text languages.
- Metadata-only artifact representation for unsuitable files.
- Snapshot/provenance metadata.
- One useful structure visualization.
- SiYuan-ready portable package.
- CTS-defined usable CLI contract by release time.
- Verification that the source tree remains untouched.

### Expected at This Maturity

- Clear failure messages.
- Basic path/input validation.
- Inspectable generated package.
- Representative tests or fixtures covering the core transformation path.
- Human-readable default CLI behavior.
- Automation-detectable success/failure behavior under CTS.

### Optional for v0.1.0

- Additional visualizations.
- Broader language detection.
- Richer project classification heuristics.
- Optional machine-readable generation summary beyond the minimum CTS contract.
- Additional metadata fields that do not complicate the core model.

### Deliberately Deferred

All native SiYuan structures and the broader future-capability list in Design Boundaries.

### Evidence That Proves v0.1.0 Complete

A release candidate should be able to demonstrate, using at least one representative software repository containing source, documentation, configuration, tests, and at least one binary or non-readable artifact, that:

1. generation completes successfully;
2. source bytes and source tree structure are not modified;
3. the produced package is inspectable before import;
4. the package imports into SiYuan through the supported portable path;
5. the resulting project root, directory surfaces, indexes, file representations, provenance, and visualization are usable;
6. the core CLI behavior matches its CTS command contract; and
7. no deferred native-SiYuan capability is required to satisfy the above.

Future improvements discovered during evaluation are evolution findings and do not lower v0.1.0 completion unless they expose a missing, broken, contradictory, unsafe, or unverified requirement above.

## Roadmap

### Phase 1 — Data Spine

- Establish project identity and CTS command boundary.
- Define discovery inputs and exclusions needed for safe scanning.
- Define the normalized project model.
- Define portable generated-resource categories.
- Define snapshot/provenance record shape.
- Prove the model against representative directory fixtures.

### Phase 2 — Core Workflows

- Implement discovery into the normalized model.
- Implement portable project overview generation.
- Implement project and directory indexes.
- Implement Markdown preservation, source wrappers, and metadata-only artifact records.
- Implement the first structure visualization.
- Assemble a SiYuan-ready portable package.

### Phase 3 — Verification

- Verify source immutability.
- Verify classifications and fallback behavior.
- Verify representative package import into SiYuan.
- Verify navigation and readability at project, directory, and file/artifact levels.
- Add CTS examples, exit-code behavior, and machine-readable output if provided by the command contract.

### Phase 4 — Release Readiness

- Apply CTS release blockers and required CLI artifacts.
- Finalize help/version output and command examples.
- Record v0.1.0 completion evidence.
- Package and release the command tool according to the applicable City Hall release/integrity requirements.

## Relationship to Existing Work

This project does not replace SiYuan, project filesystems, Git repositories, IDEs, project documentation, or existing City Hall project structures.

Its role is a transformation and exploration layer between an existing source tree and a SiYuan reference workspace.

The project should be able to ingest City Hall-governed repositories such as command tools, standards, and applications, but City Hall-specific layouts are examples and useful test cases rather than prerequisites for correct operation.

## Unresolved Matters Safe to Defer

The following remain intentionally unresolved because choosing them now would not materially improve proposal accuracy:

- Final implementation language.
- Exact CLI command name.
- Exact portable package layout.
- Exact classification thresholds and ignore defaults.
- Exact visualization syntax.
- Exact language-support matrix beyond the need for a representative supported set.
- Whether Git metadata beyond current commit/status is included in v0.1.
- Future native SiYuan integration design.

These should be resolved during implementation where evidence is cheaper and more concrete.

## Proposal Exit Assessment

**Readiness: READY**

The project has a defined problem, mission, operational users, boundaries, success and failure criteria, constraints, meaningful risks, technical direction, roadmap, governing delivery standard, and explicit v0.1.0 completion boundary.

Broad implementation may begin within this proposal boundary.

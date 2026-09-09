# Risk Assessment — SiYuan Project Exploration Overlay Generator

| Risk | Consequence | Control / Acceptance |
| --- | --- | --- |
| Portable SiYuan import semantics are insufficient or unstable | Generated package loses useful structure | Representative Markdown ZIP import was verified with SiYuan v3.8.3. Keep native internals deferred and retain import regression fixtures. |
| Large or mixed-content trees create excessive output | Poor performance or unusable snapshots | Classify content and fall back to metadata-only representations; exact thresholds remain implementation concerns. |
| Generated inference is mistaken for source truth | Misleading exploration | Preserve provenance and distinguish source facts from generated interpretation. |
| Traversal and output generation become tightly coupled | Fragile architecture | Require a normalized intermediate project model. |
| Scope drifts toward IDE/Git/sync/live workspace behavior | v0.1 never closes | Enforce explicit non-goals and completion boundary. |
| Portable indexes are less capable than Attribute Views | Reduced filtering compared with native integration | Accepted v0.1 tradeoff; native SiYuan integration is deferred. |
| Sensitive content is copied into the generated package | Trust/privacy exposure | Copying is inspectable, but exclusions are not yet implemented. Do not use the tool on sensitive trees before that gap closes. |
| Unsupported formats are mis-rendered | Corrupt or misleading output | Use metadata-only fallback rather than forced conversion. |
| SiYuan reserves an import-path component | A document branch is silently omitted | SiYuan's Markdown ZIP importer reserves `assets`; map it to `_assets` in package paths only and preserve the original source path visibly. |

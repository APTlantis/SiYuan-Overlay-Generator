//! Source-derived facts collected during discovery, before any output exists.

use std::path::PathBuf;

/// A normalized snapshot of one inspected source tree.
///
/// Discovery owns construction of this model. Projection consumes it to create
/// portable documents; neither phase is allowed to mutate `source_root`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectModel {
    pub source_root: PathBuf,
    pub entries: Vec<ProjectEntry>,
}

/// A resource found below the source root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectEntry {
    /// Path relative to `ProjectModel::source_root`.
    pub relative_path: PathBuf,
    pub kind: EntryKind,
    pub representation: Option<Representation>,
    pub byte_size: u64,
}

/// Structural resource kinds needed before content classification is added.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    Directory,
    File,
    Symlink,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Representation {
    Markdown,
    Source,
    Text,
    Artifact,
}

impl ProjectModel {
    /// Creates a model with no discovered entries.
    pub fn empty(source_root: impl Into<PathBuf>) -> Self {
        Self {
            source_root: source_root.into(),
            entries: Vec::new(),
        }
    }

    /// Returns the count of entries of one structural kind.
    pub fn count_kind(&self, kind: EntryKind) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.kind == kind)
            .count()
    }

    pub fn count_representation(&self, representation: Representation) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.representation == Some(representation))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_model_keeps_the_source_root() {
        let model = ProjectModel::empty("fixture-project");

        assert_eq!(model.source_root, PathBuf::from("fixture-project"));
        assert!(model.entries.is_empty());
    }
}

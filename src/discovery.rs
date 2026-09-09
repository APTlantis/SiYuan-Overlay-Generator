//! Deterministic, read-only filesystem discovery.

use std::{
    fmt, fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use crate::model::{EntryKind, ProjectEntry, ProjectModel, Representation};

const INLINE_LIMIT_BYTES: u64 = 1_048_576;

/// Discovers structural facts from `source_root` without following symlinks or
/// creating, changing, or deleting source-tree content.
pub fn discover(source_root: impl AsRef<Path>) -> Result<ProjectModel, DiscoveryError> {
    let source_root = source_root.as_ref();
    let root_metadata =
        fs::symlink_metadata(source_root).map_err(|source| DiscoveryError::ReadRoot {
            path: source_root.to_path_buf(),
            source,
        })?;

    if !root_metadata.is_dir() {
        return Err(DiscoveryError::NotDirectory {
            path: source_root.to_path_buf(),
        });
    }

    let canonical_root =
        fs::canonicalize(source_root).map_err(|source| DiscoveryError::ReadRoot {
            path: source_root.to_path_buf(),
            source,
        })?;
    let mut model = ProjectModel::empty(canonical_root.clone());
    discover_directory(&canonical_root, &canonical_root, &mut model.entries)?;
    model
        .entries
        .sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    Ok(model)
}

fn discover_directory(
    root: &Path,
    directory: &Path,
    entries: &mut Vec<ProjectEntry>,
) -> Result<(), DiscoveryError> {
    let mut paths = fs::read_dir(directory)
        .map_err(|source| DiscoveryError::ReadDirectory {
            path: directory.to_path_buf(),
            source,
        })?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|source| DiscoveryError::ReadDirectory {
            path: directory.to_path_buf(),
            source,
        })?;
    paths.sort();

    for path in paths {
        let metadata = fs::symlink_metadata(&path).map_err(|source| DiscoveryError::ReadEntry {
            path: path.clone(),
            source,
        })?;
        let kind = entry_kind(&metadata.file_type());
        let relative_path = path
            .strip_prefix(root)
            .expect("discovery paths always remain below the discovery root")
            .to_path_buf();

        let byte_size = metadata.len();
        let representation = if kind == EntryKind::File {
            Some(
                classify_file(&path, byte_size).map_err(|source| DiscoveryError::ReadEntry {
                    path: path.clone(),
                    source,
                })?,
            )
        } else {
            None
        };
        entries.push(ProjectEntry {
            relative_path,
            kind,
            representation,
            byte_size,
        });

        if kind == EntryKind::Directory {
            discover_directory(root, &path, entries)?;
        }
    }

    Ok(())
}

fn classify_file(path: &Path, byte_size: u64) -> io::Result<Representation> {
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    if matches!(
        extension,
        "bin" | "dat" | "exe" | "dll" | "png" | "jpg" | "jpeg" | "gif" | "zip" | "pdf"
    ) || byte_size > INLINE_LIMIT_BYTES
    {
        return Ok(Representation::Artifact);
    }
    let mut sample = Vec::new();
    fs::File::open(path)?.take(8_192).read_to_end(&mut sample)?;
    if sample.contains(&0) || std::str::from_utf8(&sample).is_err() {
        return Ok(Representation::Artifact);
    }
    if matches!(extension, "md" | "markdown") {
        Ok(Representation::Markdown)
    } else if matches!(
        extension,
        "rs" | "py"
            | "js"
            | "ts"
            | "tsx"
            | "jsx"
            | "c"
            | "h"
            | "cpp"
            | "hpp"
            | "cs"
            | "java"
            | "go"
            | "rb"
            | "php"
            | "swift"
            | "kt"
            | "sh"
            | "ps1"
    ) {
        Ok(Representation::Source)
    } else {
        Ok(Representation::Text)
    }
}

fn entry_kind(file_type: &fs::FileType) -> EntryKind {
    if file_type.is_dir() {
        EntryKind::Directory
    } else if file_type.is_file() {
        EntryKind::File
    } else if file_type.is_symlink() {
        EntryKind::Symlink
    } else {
        EntryKind::Other
    }
}

/// A source-tree discovery failure with the path that caused it.
#[derive(Debug)]
pub enum DiscoveryError {
    ReadRoot { path: PathBuf, source: io::Error },
    NotDirectory { path: PathBuf },
    ReadDirectory { path: PathBuf, source: io::Error },
    ReadEntry { path: PathBuf, source: io::Error },
}

impl fmt::Display for DiscoveryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadRoot { path, source } => {
                write!(formatter, "cannot inspect `{}`: {source}", path.display())
            }
            Self::NotDirectory { path } => write!(
                formatter,
                "input path `{}` is not a directory",
                path.display()
            ),
            Self::ReadDirectory { path, source } => {
                write!(
                    formatter,
                    "cannot read directory `{}`: {source}",
                    path.display()
                )
            }
            Self::ReadEntry { path, source } => {
                write!(formatter, "cannot inspect `{}`: {source}", path.display())
            }
        }
    }
}

impl std::error::Error for DiscoveryError {}

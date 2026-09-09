//! Projection of a normalized model into a portable Markdown ZIP.

use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    discovery::{DiscoveryError, discover},
    model::{EntryKind, ProjectEntry, ProjectModel, Representation},
    zip::write_store_zip,
};

#[derive(Debug)]
pub struct GenerationReport {
    pub output_path: PathBuf,
    pub document_count: usize,
}

pub fn generate(
    source: impl AsRef<Path>,
    output: impl AsRef<Path>,
) -> Result<GenerationReport, GenerationError> {
    let model = discover(source)?;
    let output = output.as_ref().to_path_buf();
    if output.extension().and_then(|value| value.to_str()) != Some("zip") {
        return Err(GenerationError::OutputMustBeZip(output));
    }
    let file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&output)
        .map_err(|source| GenerationError::CreateOutput {
            path: output.clone(),
            source,
        })?;
    let files = render(&model)?;
    write_store_zip(file, &files).map_err(|source| GenerationError::WriteOutput {
        path: output.clone(),
        source,
    })?;
    Ok(GenerationReport {
        output_path: output,
        document_count: files.len(),
    })
}

fn render(model: &ProjectModel) -> Result<Vec<(String, Vec<u8>)>, GenerationError> {
    let root = format!(
        "{}/",
        safe_name(
            model
                .source_root
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("project")
        )
    );
    let mut files = vec![];
    let generated = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    files.push((format!("{root}README.md"), format!("# Project Overview\n\nThis is a derived, read-only reference snapshot. The source tree remains authoritative.\n\n- Source: `{}`\n- Generated at: Unix epoch seconds `{generated}`\n- Entries: {}\n- [Project index](Project Index.md)\n- [Directory index](Directory Index.md)\n- [Structure map](Structure.md)\n", model.source_root.display(), model.entries.len()).into_bytes()));
    files.push((
        format!("{root}Project Index.md"),
        render_project_index(model).into_bytes(),
    ));
    files.push((
        format!("{root}Directory Index.md"),
        render_directory_index(model).into_bytes(),
    ));
    files.push((
        format!("{root}Structure.md"),
        render_structure(model).into_bytes(),
    ));
    for entry in model
        .entries
        .iter()
        .filter(|entry| entry.kind == EntryKind::Directory)
    {
        files.push((
            format!(
                "{root}Directories/{}/README.md",
                path_string(&entry.relative_path)
            ),
            render_directory(model, entry).into_bytes(),
        ));
    }
    for entry in model
        .entries
        .iter()
        .filter(|entry| entry.kind == EntryKind::File)
    {
        files.push((
            format!("{root}{}", output_name(entry)),
            render_resource(model, entry)?.into_bytes(),
        ));
    }
    Ok(files)
}

fn render_project_index(model: &ProjectModel) -> String {
    let mut output = String::from(
        "# Project Index\n\n| Resource | Kind | Representation |\n| --- | --- | --- |\n",
    );
    for entry in model
        .entries
        .iter()
        .filter(|entry| entry.kind == EntryKind::File)
    {
        output.push_str(&format!(
            "| [{}]({}) | file | {} |\n",
            path_string(&entry.relative_path),
            output_name(entry),
            label(entry.representation.expect("file representation"))
        ));
    }
    output
}

fn render_directory_index(model: &ProjectModel) -> String {
    let mut output = String::from("# Directory Index\n\n");
    for entry in model
        .entries
        .iter()
        .filter(|entry| entry.kind == EntryKind::Directory)
    {
        output.push_str(&format!(
            "- [{}](Directories/{}/README.md)\n",
            path_string(&entry.relative_path),
            path_string(&entry.relative_path)
        ));
    }
    output
}

fn render_directory(model: &ProjectModel, directory: &ProjectEntry) -> String {
    let directory_path = &directory.relative_path;
    let mut output = format!(
        "# Directory: {}\n\nThis page is generated from source-tree structure.\n\n| Child | Kind |\n| --- | --- |\n",
        path_string(directory_path)
    );
    for entry in &model.entries {
        if entry.relative_path.parent() == Some(directory_path.as_path()) {
            let destination = if entry.kind == EntryKind::Directory {
                format!(
                    "../../Directories/{}/README.md",
                    path_string(&entry.relative_path)
                )
            } else {
                format!("../../{}", output_name(entry))
            };
            output.push_str(&format!(
                "| [{}]({destination}) | {} |\n",
                path_string(&entry.relative_path),
                if entry.kind == EntryKind::Directory {
                    "directory"
                } else {
                    "file"
                }
            ));
        }
    }
    output
}

fn render_resource(model: &ProjectModel, entry: &ProjectEntry) -> Result<String, GenerationError> {
    let path = model.source_root.join(&entry.relative_path);
    let representation = entry.representation.expect("file representation");
    let title = path_string(&entry.relative_path);
    match representation {
        Representation::Markdown => Ok(fs::read_to_string(&path)
            .map_err(|source| GenerationError::ReadSource { path, source })?),
        Representation::Source | Representation::Text => {
            let content = fs::read_to_string(&path)
                .map_err(|source| GenerationError::ReadSource { path, source })?;
            let fence = if representation == Representation::Source {
                language(&entry.relative_path)
            } else {
                "text"
            };
            Ok(format!(
                "# {title}\n\nDerived wrapper for a source-authoritative file.\n\n```{fence}\n{content}\n```\n"
            ))
        }
        Representation::Artifact => Ok(format!(
            "# Artifact: {title}\n\nThis resource is metadata-only because it is binary, unsupported, or exceeds the inline safety limit.\n\n- Source path: `{title}`\n- Size: {} bytes\n- Representation: artifact metadata only\n",
            entry.byte_size
        )),
    }
}

fn render_structure(model: &ProjectModel) -> String {
    let mut output = String::from("# Structure Map\n\n```text\n.");
    for entry in &model.entries {
        output.push_str(&format!(
            "\n{}{}",
            "  ".repeat(entry.relative_path.components().count().saturating_sub(1)),
            path_string(&entry.relative_path)
        ));
    }
    output.push_str("\n```\n");
    output
}

fn output_name(entry: &ProjectEntry) -> String {
    match entry.representation.expect("file representation") {
        Representation::Markdown => format!("Documents/{}", path_string(&entry.relative_path)),
        Representation::Source | Representation::Text => {
            format!("Resources/{}.md", path_string(&entry.relative_path))
        }
        Representation::Artifact => format!("Artifacts/{}.md", path_string(&entry.relative_path)),
    }
}
fn path_string(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
fn safe_name(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '-'
            }
        })
        .collect()
}
fn label(value: Representation) -> &'static str {
    match value {
        Representation::Markdown => "Markdown",
        Representation::Source => "source wrapper",
        Representation::Text => "text wrapper",
        Representation::Artifact => "artifact metadata",
    }
}
fn language(path: &Path) -> &'static str {
    match path.extension().and_then(|value| value.to_str()) {
        Some("rs") => "rust",
        Some("py") => "python",
        Some("js") => "javascript",
        Some("ts") => "typescript",
        Some("toml") => "toml",
        Some("json") => "json",
        Some("yml" | "yaml") => "yaml",
        Some("ps1") => "powershell",
        _ => "text",
    }
}

#[derive(Debug)]
pub enum GenerationError {
    Discovery(DiscoveryError),
    OutputMustBeZip(PathBuf),
    CreateOutput { path: PathBuf, source: io::Error },
    ReadSource { path: PathBuf, source: io::Error },
    WriteOutput { path: PathBuf, source: io::Error },
}
impl From<DiscoveryError> for GenerationError {
    fn from(value: DiscoveryError) -> Self {
        Self::Discovery(value)
    }
}
impl fmt::Display for GenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Discovery(error) => write!(f, "{error}"),
            Self::OutputMustBeZip(path) => {
                write!(f, "output `{}` must end in .zip", path.display())
            }
            Self::CreateOutput { path, source } => {
                write!(f, "cannot create output `{}`: {source}", path.display())
            }
            Self::ReadSource { path, source } => {
                write!(f, "cannot read source `{}`: {source}", path.display())
            }
            Self::WriteOutput { path, source } => {
                write!(f, "cannot write output `{}`: {source}", path.display())
            }
        }
    }
}
impl std::error::Error for GenerationError {}

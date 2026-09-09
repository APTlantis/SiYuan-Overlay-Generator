use std::{fs, path::PathBuf};

use siyuan_overlay_generator::{discovery::discover, model::EntryKind};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/representative-project")
}

#[test]
fn discovery_is_deterministic_and_leaves_the_fixture_unchanged() {
    let fixture = fixture_path();
    let before = fixture_fingerprint(&fixture);

    let model = discover(&fixture).expect("fixture must be readable");

    assert_eq!(fixture_fingerprint(&fixture), before);
    assert_eq!(model.count_kind(EntryKind::Directory), 5);
    assert_eq!(model.count_kind(EntryKind::File), 6);
    assert_eq!(
        model
            .entries
            .iter()
            .map(|entry| entry.relative_path.clone())
            .collect::<Vec<_>>(),
        vec![
            PathBuf::from("README.md"),
            PathBuf::from("assets"),
            PathBuf::from("assets/logo.dat"),
            PathBuf::from("config"),
            PathBuf::from("config/tool.toml"),
            PathBuf::from("src"),
            PathBuf::from("src/main.rs"),
            PathBuf::from("tests"),
            PathBuf::from("tests/smoke.rs"),
            PathBuf::from("vendor"),
            PathBuf::from("vendor/opaque.bin"),
        ]
    );
}

#[test]
fn discovery_rejects_a_file_as_the_source_root() {
    let error =
        discover(fixture_path().join("README.md")).expect_err("a file cannot be a source tree");

    assert!(error.to_string().contains("is not a directory"));
}

fn fixture_fingerprint(root: &std::path::Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut files = Vec::new();
    collect_files(root, root, &mut files);
    files.sort_by(|left, right| left.0.cmp(&right.0));
    files
}

fn collect_files(
    root: &std::path::Path,
    directory: &std::path::Path,
    files: &mut Vec<(PathBuf, Vec<u8>)>,
) {
    for entry in fs::read_dir(directory).expect("fixture directory is readable") {
        let path = entry.expect("fixture entry is readable").path();
        if path.is_dir() {
            collect_files(root, &path, files);
        } else {
            files.push((
                path.strip_prefix(root)
                    .expect("fixture path remains below root")
                    .to_path_buf(),
                fs::read(&path).expect("fixture file is readable"),
            ));
        }
    }
}

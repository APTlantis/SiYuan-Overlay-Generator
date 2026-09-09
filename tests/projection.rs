use std::{fs, path::PathBuf};

use siyuan_overlay_generator::projection::generate;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/representative-project")
}

#[test]
fn generation_creates_an_inspectable_markdown_zip_without_changing_source() {
    let fixture = fixture_path();
    let before = fingerprint(&fixture);
    let output = std::env::temp_dir().join(format!(
        "siyuan-overlay-generator-{}-{}.zip",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock is after epoch")
            .as_nanos()
    ));

    let report = generate(&fixture, &output).expect("fixture generation succeeds");
    let package = fs::read(&output).expect("package is readable");

    assert_eq!(fingerprint(&fixture), before);
    assert_eq!(report.document_count, 15);
    assert!(package.starts_with(b"PK\x03\x04"));
    assert!(
        package
            .windows(b"README.md".len())
            .any(|window| window == b"README.md")
    );
    assert!(
        package
            .windows(b"Project Index.md".len())
            .any(|window| window == b"Project Index.md")
    );
    assert!(
        package
            .windows(b"Artifacts/vendor/opaque.bin.md".len())
            .any(|window| window == b"Artifacts/vendor/opaque.bin.md")
    );
    fs::remove_file(output).expect("temporary package is removable");
}

fn fingerprint(root: &std::path::Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut files = Vec::new();
    collect(root, root, &mut files);
    files.sort_by(|left, right| left.0.cmp(&right.0));
    files
}

fn collect(
    root: &std::path::Path,
    directory: &std::path::Path,
    files: &mut Vec<(PathBuf, Vec<u8>)>,
) {
    for entry in fs::read_dir(directory).expect("fixture directory is readable") {
        let path = entry.expect("fixture entry is readable").path();
        if path.is_dir() {
            collect(root, &path, files);
        } else {
            files.push((
                path.strip_prefix(root)
                    .expect("path is below root")
                    .to_path_buf(),
                fs::read(&path).expect("fixture file is readable"),
            ));
        }
    }
}

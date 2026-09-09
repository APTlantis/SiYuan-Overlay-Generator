use siyuan_overlay_generator::{
    discovery::discover,
    model::{EntryKind, ProjectModel},
};

const HELP: &str = "SiYuan Project Exploration Overlay Generator\n\nUsage:\n  siyuan-overlay-generator inspect <SOURCE>\n\nCommands:\n  inspect <SOURCE>  Read a directory tree into the normalized project model.\n\nThe inspect command is read-only and reports structural discovery only.\nPackage generation is not implemented yet.\n\nOptions:\n  -h, --help       Print help\n  -V, --version    Print version";

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("-h" | "--help") | None => println!("{HELP}"),
        Some("-V" | "--version") => println!("{}", env!("CARGO_PKG_VERSION")),
        Some("inspect") => inspect(std::env::args().nth(2)),
        Some(_) => {
            eprintln!("unknown command; run --help for usage");
            std::process::exit(2);
        }
    }
}

fn inspect(source: Option<String>) {
    let Some(source) = source else {
        eprintln!("inspect requires a source directory");
        std::process::exit(2);
    };

    match discover(&source) {
        Ok(model) => print_summary(&model),
        Err(error) => {
            eprintln!("discovery failed: {error}");
            std::process::exit(1);
        }
    }
}

fn print_summary(model: &ProjectModel) {
    println!("Source: {}", model.source_root.display());
    println!("Entries: {}", model.entries.len());
    println!("Directories: {}", model.count_kind(EntryKind::Directory));
    println!("Files: {}", model.count_kind(EntryKind::File));
    println!("Symlinks: {}", model.count_kind(EntryKind::Symlink));
    println!("Other: {}", model.count_kind(EntryKind::Other));
    println!("No package was generated; inspect is read-only.");
}

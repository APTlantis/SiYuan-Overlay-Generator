const HELP: &str = "SiYuan Project Exploration Overlay Generator\n\nUsage:\n  siyuan-overlay-generator <SOURCE> [OPTIONS]\n\nThis Rust CLI is in its bootstrap phase. Generation is not implemented yet.\nIt will inspect source trees read-only and produce a portable SiYuan import package.\n\nOptions:\n  -h, --help       Print help\n  -V, --version    Print version";

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("-h" | "--help") | None => println!("{HELP}"),
        Some("-V" | "--version") => println!("{}", env!("CARGO_PKG_VERSION")),
        Some(_) => {
            eprintln!("generation is not implemented yet; run --help for the planned interface");
            std::process::exit(2);
        }
    }
}

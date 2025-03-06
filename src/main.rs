use std::path::Path;
mod vscode_settings;

fn main() -> std::io::Result<()> {
    vscode_settings::create(Path::new("."))
}

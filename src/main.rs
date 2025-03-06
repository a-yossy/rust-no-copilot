use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

const COPILOT_DISABLED_SETTING: &str = r#"{
  "editor.inlineSuggest.enabled": false,
}
"#;

fn main() -> std::io::Result<()> {
    let vscode_dir = Path::new(".vscode");
    create_dir_all(vscode_dir)?;

    let settings_file = vscode_dir.join("settings.json");
    let mut file = File::create(settings_file)?;
    file.write_all(COPILOT_DISABLED_SETTING.as_bytes())?;

    Ok(())
}

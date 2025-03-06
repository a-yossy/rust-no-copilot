use std::fs::{self, create_dir_all, read_to_string};
use std::path::Path;

use serde_json::{json, Value};

const COPILOT_DISABLED_SETTING_KEY: &str = "editor.inlineSuggest.enabled";

pub fn create(base_dir: &Path) -> std::io::Result<()> {
    let vscode_dir = base_dir.join(".vscode");
    create_dir_all(&vscode_dir)?;
    let copilot_disabled_setting = serde_json::to_string_pretty(&json!({
        COPILOT_DISABLED_SETTING_KEY: false
    }))
    .unwrap();

    let settings_file = vscode_dir.join("settings.json");
    if settings_file.exists() {
        let settings_file_content = read_to_string(&settings_file).unwrap();
        let mut settings_file_content: Value = serde_json::from_str(&settings_file_content)?;
        settings_file_content[COPILOT_DISABLED_SETTING_KEY] = Value::Bool(false);
        fs::write(
            settings_file,
            serde_json::to_string_pretty(&settings_file_content)?,
        )?;

        return Ok(());
    }
    fs::write(settings_file, copilot_disabled_setting)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn vscodeディレクトリが存在しない場合_設定ファイルが作成されること() {
        let temp_dir = tempdir().unwrap();
        let settings_json = temp_dir.path().join(".vscode").join("settings.json");
        assert!(!settings_json.exists());

        let result = create(temp_dir.path());

        assert!(result.is_ok());
        let settings_json_content = fs::read_to_string(settings_json).unwrap();
        assert_eq!(
            settings_json_content,
            r#"{
  "editor.inlineSuggest.enabled": false
}"#
        );
    }

    #[test]
    fn vscodeディレクトリが存在するかつ設定ファイルが存在しない場合_設定ファイルが作成されること() {
        let temp_dir = tempdir().unwrap();
        let vscode_dir = temp_dir.path().join(".vscode");
        let _ = create_dir_all(&vscode_dir);
        let settings_json = vscode_dir.join("settings.json");

        let result = create(temp_dir.path());

        assert!(result.is_ok());
        let settings_json_content = fs::read_to_string(settings_json).unwrap();
        assert_eq!(
            settings_json_content,
            r#"{
  "editor.inlineSuggest.enabled": false
}"#
        );
    }
}

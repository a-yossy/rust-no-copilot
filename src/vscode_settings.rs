use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

pub const COPILOT_DISABLED_SETTING: &str = r#"{
      "editor.inlineSuggest.enabled": false,
    }
    "#;

pub fn create(base_dir: &Path) -> std::io::Result<()> {
    let vscode_dir = base_dir.join(".vscode");
    create_dir_all(&vscode_dir)?;

    let settings_file = vscode_dir.join("settings.json");
    let mut file = File::create(settings_file)?;
    file.write_all(COPILOT_DISABLED_SETTING.as_bytes())?;

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
        assert_eq!(settings_json_content, COPILOT_DISABLED_SETTING);
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
        assert_eq!(settings_json_content, COPILOT_DISABLED_SETTING);
    }
}

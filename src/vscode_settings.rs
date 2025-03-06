use std::fs::{self, create_dir_all, read_to_string};
use std::path::Path;

use serde_json::{json, Value};

const COPILOT_DISABLED_SETTING_KEY: &str = "editor.inlineSuggest.enabled";

pub fn create(base_dir: &Path) -> std::io::Result<()> {
    let vscode_dir = base_dir.join(".vscode");
    create_dir_all(&vscode_dir)?;
    let settings_path = vscode_dir.join("settings.json");

    if settings_path.exists() {
        let settings_json = read_to_string(&settings_path)?;
        let mut settings_json: Value = serde_json::from_str(&settings_json)?;
        settings_json[COPILOT_DISABLED_SETTING_KEY] = json!(false);
        fs::write(
            settings_path,
            serde_json::to_string_pretty(&settings_json)? + "\n",
        )?;
    } else {
        let copilot_disabled_setting = serde_json::json!({
            COPILOT_DISABLED_SETTING_KEY: false
        });
        fs::write(
            settings_path,
            serde_json::to_string_pretty(&copilot_disabled_setting)? + "\n",
        )?;
    }

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
}
"#
        );
    }

    #[test]
    fn vscodeディレクトリが存在する_設定ファイルが存在しない場合_設定ファイルが作成されること() {
        let temp_dir = tempdir().unwrap();
        let vscode_dir = temp_dir.path().join(".vscode");
        create_dir_all(&vscode_dir).unwrap();
        let settings_json = vscode_dir.join("settings.json");

        let result = create(temp_dir.path());

        assert!(result.is_ok());
        let settings_json_content = fs::read_to_string(settings_json).unwrap();
        assert_eq!(
            settings_json_content,
            r#"{
  "editor.inlineSuggest.enabled": false
}
"#
        );
    }

    #[test]
    fn vscodeディレクトリが存在する_設定ファイルが存在する_設定ファイルが正しい形式の場合_設定ファイルが上書きされること(
    ) {
        let temp_dir = tempdir().unwrap();
        let vscode_dir = temp_dir.path().join(".vscode");
        create_dir_all(&vscode_dir).unwrap();
        let settings_json = vscode_dir.join("settings.json");
        let _ = fs::write(
            &settings_json,
            r#"{
  "test": false
}"#,
        );

        let result = create(temp_dir.path());

        assert!(result.is_ok());
        let settings_json_content = fs::read_to_string(settings_json).unwrap();
        assert_eq!(
            settings_json_content,
            r#"{
  "editor.inlineSuggest.enabled": false,
  "test": false
}
"#
        );
    }

    #[test]
    fn vscodeディレクトリが存在する_設定ファイルが存在する_設定ファイルが不正な形式の場合_エラーが返ること(
    ) {
        let temp_dir = tempdir().unwrap();
        let vscode_dir = temp_dir.path().join(".vscode");
        create_dir_all(&vscode_dir).unwrap();
        let settings_json = vscode_dir.join("settings.json");
        let _ = fs::write(
            &settings_json,
            r#"{
  "test":
}"#,
        );

        let result = create(temp_dir.path());

        assert!(result.is_err());
        let settings_json_content = fs::read_to_string(settings_json).unwrap();
        assert_eq!(
            settings_json_content,
            r#"{
  "test":
}"#
        );
    }
}

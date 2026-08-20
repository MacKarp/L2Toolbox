use crate::profiles::types::ProfileEntry;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub profile_name: String,
    pub lineage2_path: String,
    #[serde(
        default,
        deserialize_with = "empty_string_is_none",
        serialize_with = "none_as_empty_string"
    )]
    pub system_path: Option<String>,
    pub custom_subdir: bool,
}

impl ProfileConfig {
    pub fn save(&self, profile_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(profile_dir)?;

        let file_path = profile_dir.join("profile_config.toml");
        let tmp_path = profile_dir.join("profile_config.toml.tmp");

        let toml = toml::to_string_pretty(self)?;

        fs::write(&tmp_path, toml)?;

        // Retry rename (Windows lock workaround)
        for attempt in 1..=3 {
            match fs::rename(&tmp_path, &file_path) {
                Ok(_) => {
                    println!("✅ Config saved to {file_path:?}");
                    return Ok(());
                }

                Err(e) if e.raw_os_error() == Some(5) || e.raw_os_error() == Some(32) => {
                    eprintln!("⚠️ Attempt #{attempt}: file locked — retrying...");
                    thread::sleep(Duration::from_millis(300));
                }

                Err(e) => return Err(Box::new(e)),
            }
        }

        Err("❌ Failed to save config: file locked".into())
    }
}

pub fn load(profile: &ProfileEntry) -> Result<ProfileConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(&profile.config_path)?;

    let config: ProfileConfig = toml::from_str(&content)?;

    if config.profile_name.trim().is_empty() {
        return Err("❌ profile_name cannot be empty".into());
    }

    if config.lineage2_path.trim().is_empty() {
        return Err("❌ lineage2_path cannot be empty".into());
    }

    Ok(config)
}

fn empty_string_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.trim().is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}
fn none_as_empty_string<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(v),
        None => serializer.serialize_str(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::types::ProfileEntry;
    use tempfile::tempdir;

    fn test_profile_config() -> ProfileConfig {
        ProfileConfig {
            profile_name: "Test Profile".to_string(),
            lineage2_path: r"C:\Games\Lineage2".to_string(),
            system_path: Some(r"C:\Games\Lineage2\custom-system".to_string()),
            custom_subdir: true,
        }
    }

    fn test_profile_entry(directory: &Path) -> ProfileEntry {
        ProfileEntry::new(
            "test-profile-id".to_string(),
            "Test Profile".to_string(),
            directory.to_path_buf(),
        )
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let temp_dir = tempdir().unwrap();
        let profile_dir = temp_dir.path().join("profile");

        let original = test_profile_config();
        original.save(&profile_dir).unwrap();

        let profile = test_profile_entry(&profile_dir);
        let loaded = load(&profile).unwrap();

        assert_eq!(loaded.profile_name, original.profile_name);
        assert_eq!(loaded.lineage2_path, original.lineage2_path);
        assert_eq!(loaded.system_path, original.system_path);
        assert_eq!(loaded.custom_subdir, original.custom_subdir);
    }

    #[test]
    fn test_save_creates_profile_directory() {
        let temp_dir = tempdir().unwrap();
        let profile_dir = temp_dir.path().join("profile");

        let config = test_profile_config();
        config.save(&profile_dir).unwrap();

        assert!(profile_dir.is_dir());
        assert!(profile_dir.join("profile_config.toml").is_file());
    }

    #[test]
    fn test_load_rejects_empty_profile_name() {
        let temp_dir = tempdir().unwrap();
        let profile_dir = temp_dir.path().join("profile");
        fs::create_dir_all(&profile_dir).unwrap();

        fs::write(
            profile_dir.join("profile_config.toml"),
            r#"
                profile_name = ""
                lineage2_path = 'C:\Games\Lineage2'
                system_path = ""
                custom_subdir = false
            "#,
        )
        .unwrap();

        let profile = test_profile_entry(&profile_dir);
        let result = load(&profile);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "❌ profile_name cannot be empty"
        );
    }

    #[test]
    fn test_load_rejects_empty_lineage2_path() {
        let temp_dir = tempdir().unwrap();
        let profile_dir = temp_dir.path().join("profile");
        fs::create_dir_all(&profile_dir).unwrap();

        fs::write(
            profile_dir.join("profile_config.toml"),
            r#"
                profile_name = "Test Profile"
                lineage2_path = ""
                system_path = ""
                custom_subdir = false
            "#,
        )
        .unwrap();

        let profile = test_profile_entry(&profile_dir);
        let result = load(&profile);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "❌ lineage2_path cannot be empty"
        );
    }

    #[test]
    fn test_empty_system_path_deserializes_as_none() {
        let toml = r#"
            profile_name = "Test Profile"
            lineage2_path = 'C:\Games\Lineage2'
            system_path = ""
            custom_subdir = false
        "#;

        let config: ProfileConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.system_path, None);
    }
}

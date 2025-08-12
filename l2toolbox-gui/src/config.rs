use chrono::Local;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub last_profile: String,
    pub language: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            last_profile: "".to_string(),
            language: "En".to_string(),
        }
    }
}

impl Config {
    pub fn load_or_create() -> Result<Config, Box<dyn std::error::Error>> {
        println!("Loading config!");
        let project_dirs = ProjectDirs::from("", "", "L2Toolbox")
            .ok_or("Can't obtain default directory paths!")?;

        let config_dir = project_dirs.config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let file_path = config_dir.join("config.toml");
        if file_path.exists() {
            println!("Loading existing file: {:?}", file_path);
            Config::load_config(&file_path)
        } else {
            println!("Creating new file: {:?}", file_path);
            Config::create_default_config(&file_path)?;
            Config::load_config(&file_path)
        }
    }

    fn load_config(file_path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
        let toml_file = fs::read_to_string(file_path)?;

        match toml::from_str::<Config>(&toml_file) {
            Ok(config) => Ok(config),
            Err(e) => {
                eprintln!("Failed to parse config file: {}", e);

                let timestamp = Local::now().format("%Y%m%d_%H%M%S");
                let backup_path =
                    file_path.with_file_name(format!("config.toml_{}.bak", timestamp));

                fs::rename(file_path, backup_path)?;

                Self::create_default_config(file_path)?;

                Self::load_config(file_path)
            }
        }
    }

    fn create_default_config(file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::default();
        let toml_file = toml::to_string(&config)?;
        fs::write(file_path, toml_file)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_default_config_values() {
        let config = Config::default();
        assert_eq!(config.last_profile, "");
        assert_eq!(config.language, "En");
    }

    #[test]
    fn test_create_default_config_writes_file() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.toml");

        Config::create_default_config(&config_path).unwrap();
        let contents = fs::read_to_string(&config_path).unwrap();

        assert!(contents.contains("last_profile = \"\""));
        assert!(contents.contains("language = \"En\""));
    }

    #[test]
    fn test_load_config_valid_file() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.toml");

        let config_data = r#"
        last_profile = "test_user"
        language = "Pl"
    "#;
        fs::write(&config_path, config_data).unwrap();

        let config = Config::load_config(&config_path).unwrap();
        assert_eq!(config.last_profile, "test_user");
        assert_eq!(config.language, "Pl");
    }

    #[test]
    fn test_create_and_load_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.toml");

        // Create default config
        Config::create_default_config(&config_path).unwrap();

        // Load config
        let loaded_config = Config::load_config(&config_path).unwrap();
        assert_eq!(loaded_config.last_profile, "");
        assert_eq!(loaded_config.language, "En");
    }

    #[test]
    fn test_corrupted_config_recovery() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.toml");

        // Write corrupted content
        fs::write(&config_path, "not a valid toml").unwrap();

        // Attempt to load, should recover
        let recovered_config = Config::load_config(&config_path).unwrap();
        assert_eq!(recovered_config.last_profile, "");
        assert_eq!(recovered_config.language, "En");

        // Check that backup file was created
        let backup_files: Vec<_> = fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|entry| {
                let path = entry.unwrap().path();
                if path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("config.toml_")
                    && path.extension().map_or(false, |ext| ext == "bak")
                {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        assert!(!backup_files.is_empty(), "Backup file was not created");
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let original = Config {
            last_profile: "user123".to_string(),
            language: "Pl".to_string(),
        };

        let toml_str = toml::to_string(&original).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(original.last_profile, deserialized.last_profile);
        assert_eq!(original.language, deserialized.language);
    }
}

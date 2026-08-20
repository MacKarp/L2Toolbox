use crate::profiles::types::ProfileEntry;
use directories::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};

/// Get the base `profiles` directory for the application.
fn get_profiles_dir() -> Result<PathBuf, String> {
    let project_dirs =
        ProjectDirs::from("", "", "L2Toolbox").ok_or("Can't obtain default directory paths!")?;
    let config_dir = project_dirs.config_dir();

    let profiles_dir = config_dir
        .parent()
        .ok_or("❌ Cannot determine parent of config_dir")?
        .join("profiles");

    Ok(profiles_dir)
}

fn ensure_profiles_dir(path: &Path) -> Result<PathBuf, String> {
    if !path.exists() {
        println!("ℹ️ Creating profiles directory: {path:?}");

        fs::create_dir_all(path)
            .map_err(|e| format!("❌ Failed to create profiles directory: {e}"))?;
    }

    if !path.is_dir() {
        return Err(format!(
            "❌ Profiles path exists but is not a directory: {path:?}"
        ));
    }

    Ok(path.to_path_buf())
}

fn profile_config_path(dir: &Path) -> Option<PathBuf> {
    let config = dir.join("profile_config.toml");

    config.exists().then_some(config)
}

/// Scan a given directory for valid profile directories.
fn scan_profiles_in_dir(profiles_dir: &Path) -> Result<Vec<ProfileEntry>, String> {
    let mut profile_list = Vec::new();

    for entry in fs::read_dir(profiles_dir)
        .map_err(|e| format!("❌ Failed to read profiles directory: {e}"))?
    {
        let entry = entry.map_err(|e| format!("❌ Failed to read entry: {e}"))?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let Some(config_path) = profile_config_path(&path) else {
            continue;
        };

        let content = match fs::read_to_string(&config_path) {
            Ok(content) => content,
            Err(e) => {
                println!("⚠️ Failed to read profile config {config_path:?}: {e}");
                continue;
            }
        };

        let profile_config: crate::profiles::config::ProfileConfig = match toml::from_str(&content)
        {
            Ok(config) => config,
            Err(e) => {
                println!("⚠️ Failed to parse profile config {config_path:?}: {e}");
                continue;
            }
        };

        if profile_config.profile_name.trim().is_empty() {
            println!("⚠️ Profile config {config_path:?} has no profile name");
            continue;
        }

        let id = path
            .file_name()
            .ok_or("❌ Invalid profile directory name")?
            .to_string_lossy()
            .into_owned();

        profile_list.push(ProfileEntry::new(id, profile_config.profile_name, path));
    }

    Ok(profile_list)
}

/// Scan for available profiles in the app's profile directory.
///
/// Returns a list of profile directories that contain a `profile_config.toml` file.
pub fn scan_available_profiles() -> Vec<ProfileEntry> {
    println!("ℹ️ Loading profile list!");

    let profiles_dir = match get_profiles_dir() {
        Ok(d) => d,
        Err(e) => {
            println!("⚠ {e}");
            return vec![];
        }
    };

    let profiles_dir = match ensure_profiles_dir(&profiles_dir) {
        Ok(d) => d,
        Err(e) => {
            println!("⚠ {e}");
            return vec![];
        }
    };

    match scan_profiles_in_dir(&profiles_dir) {
        Ok(list) => list,
        Err(e) => {
            println!("⚠ {e}");
            vec![]
        }
    }
}

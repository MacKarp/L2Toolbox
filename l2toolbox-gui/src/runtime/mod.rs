use crate::config::ApplicationConfig;
use crate::profiles::config::ProfileConfig;
use crate::profiles::scanner::scan_available_profiles;
use crate::profiles::types::ProfileEntry;
use directories::ProjectDirs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Runtime {
    pub application_config_dir: PathBuf,
    //pub application_data_dir: PathBuf,
    pub profile_list: Vec<ProfileEntry>,
    pub current_profile: Option<ProfileEntry>,
    pub selected_profile: Option<ProfileEntry>,

    pub loaded_profile: Option<ProfileConfig>,
    pub profile_draft: Option<ProfileConfig>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            application_config_dir: PathBuf::new(),
            //application_data_dir: PathBuf::new(),
            profile_list: Vec::new(),
            current_profile: None,
            selected_profile: None,
            loaded_profile: None,
            profile_draft: None,
        }
    }
}

impl Runtime {
    pub fn new() -> Result<Self, String> {
        let project_dirs = ProjectDirs::from("", "", "L2Toolbox")
            .ok_or_else(|| "❌ Can't obtain default directory paths".to_string())?;

        Ok(Self {
            application_config_dir: project_dirs.config_dir().to_path_buf(),
            //application_data_dir: project_dirs.data_dir().to_path_buf(),
            profile_list: Vec::new(),
            current_profile: None,
            selected_profile: None,
            loaded_profile: None,
            profile_draft: None,
        })
    }

    pub fn initialize(config: &ApplicationConfig) -> Result<Self, String> {
        let mut runtime = Self::new()?;

        println!("ℹ️ Loading profile list...");
        runtime.profile_list = scan_available_profiles();
        println!(
            "ℹ️ Found {} available profile(s).",
            runtime.profile_list.len()
        );

        match config.last_profile_id.as_deref() {
            Some(last_profile_id) => {
                println!("ℹ️ Last used profile ID: {last_profile_id}");

                runtime.current_profile = runtime
                    .profile_list
                    .iter()
                    .find(|profile| profile.id == last_profile_id)
                    .cloned();

                match &runtime.current_profile {
                    Some(profile) => {
                        println!(
                            "✅ Restored last profile: '{}' ({})",
                            profile.name, profile.id
                        );
                    }
                    None => {
                        println!("⚠️ Last used profile ID '{last_profile_id}' was not found.");
                    }
                }
            }
            None => {
                println!("ℹ️ No last used profile configured.");
            }
        }

        Ok(runtime)
    }
}

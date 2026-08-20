use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ProfileEntry {
    pub id: String,
    pub name: String,
    //pub directory: PathBuf,
    pub config_path: PathBuf,
}

impl ProfileEntry {
    pub fn new(id: String, name: String, directory: PathBuf) -> Self {
        let config_path = directory.join("profile_config.toml");

        Self {
            id,
            name,
            //directory,
            config_path,
        }
    }
}

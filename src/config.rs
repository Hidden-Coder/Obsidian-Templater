use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/**
 * Config struct
 */
#[derive(Debug,Serialize, Deserialize)]
pub struct MyConfig {
    pub template_path: Option<PathBuf>,
    pub vault_dir: Option<PathBuf>,
    pub obsidian_config: Option<PathBuf>,
}

impl std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            template_path: None,
            vault_dir: None,
            obsidian_config: None,
        }
    }
}

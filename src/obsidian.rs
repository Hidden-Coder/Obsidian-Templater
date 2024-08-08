use std::{collections::HashMap, fs::{self}, io::{self, Read, Write}, path::PathBuf, time::SystemTime};

use serde::{Serialize, Deserialize};

use crate::MyConfig;

const OBSIDIAN_CONFIG_FILE: &str = "obsidian.json";

#[derive(Debug,Serialize, Deserialize)]
pub struct Vault {
    /**
     * Absolute path to the vault directory
     */
    pub path: String,
    /**
     * Timestamp of the creation of the vault
     */
    pub ts: u128,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ObsidianConfig {
    /**
     * Map of vault id to vault
     */
    pub vaults: HashMap<String, Vault>,
    /**
     * Frame settings of the obsidian window
     */
    pub frame: String,
}

/**
 * Get the obsidian config from the obsidian.json file
 */
fn get_obsidian_config(config: &MyConfig) -> io::Result<ObsidianConfig> {
    let obsidian_path = config.obsidian_config.as_ref().expect("Obsidian config path needs to be set");
    let path_to_obsidian_config = obsidian_path.join(OBSIDIAN_CONFIG_FILE);
    let mut config_string = String::new();
    fs::File::open(path_to_obsidian_config)?.read_to_string(&mut config_string)?;
    let obsidian_config: ObsidianConfig = serde_json::from_str(&config_string)?;
    return Ok(obsidian_config);
}

/**
 * Write the new obsidian config to the obsidian.json file
 */
fn write_obsidian_config(config: &MyConfig, obs_config: ObsidianConfig) -> io::Result<()> {
    let obsidian_path = config.obsidian_config.as_ref().expect("Obsidian config path needs to be set");
    let path_to_obsidian_config = obsidian_path.join(OBSIDIAN_CONFIG_FILE);
    let new_obsidian_config =  serde_json::to_string_pretty(&obs_config)?;
    fs::write(path_to_obsidian_config, new_obsidian_config)?;

    return Ok(());
}

/**
 * Generate a new vault id.
 * This is used to identify the vault in the obsidian.json file.
 */
fn generate_vault_id() -> String {
    let id = uuid::Uuid::new_v4().as_simple().to_string();
    let mut ret = String::new();
    let _ = &id[..16].clone_into(&mut ret);
    return ret;
}

/**
 * Create a new vault with the given path.
 * This will add the new vault to the obsidian.json file.
 */
pub fn create_new_vault(config: &MyConfig, path: &PathBuf) -> io::Result<()> {
    // Write into obsidian.json
    let mut obs_config = get_obsidian_config(config)?;
    let new_id = generate_vault_id();
    let vault_path = String::from(path.to_str().unwrap());
    let new_vault = Vault {
        path: vault_path,
        ts: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards").as_millis()
    };
    obs_config.vaults.insert(new_id.clone(), new_vault);
    write_obsidian_config(config, obs_config)?;

    // Write new file in obsidian path
    let new_file_path = config.obsidian_config.as_ref().unwrap().join(format!("{}.json", new_id));
    let mut file = fs::File::create(new_file_path)?;
    let vault_config = VaultConfig::default();
    let buf = serde_json::to_string_pretty(&vault_config)?;
    file.write_all(buf.as_bytes())?;

    return Ok(());
}

/**
 * Vault config struct
 */
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct VaultConfig {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub isMaximized: bool,
    pub devTools: bool,
    pub zoom: i32,
}

impl std::default::Default for VaultConfig {
    fn default() -> Self {
        VaultConfig {
            x: 0,
            y: 0,
            width: 1024,
            height: 800,
            isMaximized: true,
            devTools: false,
            zoom: 0
        }
    }
}

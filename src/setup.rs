use std::io::BufRead;
use std::{fs, io, path::PathBuf, process::exit};

use crate::config;
use crate::errors;
use crate::{MyConfig, APP_NAME};

/**
 * Set the directory, that is used for creating new vault in.
 * This will be stored in the config file.
 */
pub fn set_vault_dir(mut config: config::MyConfig, new_path: PathBuf) {
    config.vault_dir = Some(new_path);
    if let Err(e) = confy::store(APP_NAME, None, config) {
        println!("Could not store into config");
        println!("{:?}", e);
        exit(1);
    }
}

/**
 * Set the template directory, that is used for getting the vault template from.
 * This will be stored in the config file.
 */
pub fn set_template_path(mut config: config::MyConfig, new_path: PathBuf) {
    config.template_path = Some(new_path);
    if let Err(e) = confy::store(APP_NAME, None, config) {
        println!("Could not store into config");
        println!("{:?}", e);
        exit(1);
    }
}

/**
 * Set the obsidian installation path.
 * This will be stored in the config file.
 */
pub fn set_obsidian_path(mut config: config::MyConfig, new_path: PathBuf) {
    config.obsidian_config = Some(new_path);
    if let Err(e) = confy::store(APP_NAME, None, config) {
        println!("Could not store into config");
        println!("{:?}", e);
        exit(1);
    }
}

/**
 * The flow for setup the vault dir
 */
fn setup_vault_dir(config: &mut config::MyConfig) -> Result<(), errors::Errors> {
    println!("Please provide the path in which the new vault folders will be created");
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut handle = stdin.lock();
    if let Err(e) = handle.read_line(&mut buffer) {
        return Err(errors::Errors::IOError(e));
    };
    // Set the vault dir in the config
    config.vault_dir = Some(PathBuf::from(buffer));
    if let Err(e) = confy::store(APP_NAME, None, config) {
        println!(
            "Something went wrong when trying to store the config: {:?}",
            e
        );
        return Err(errors::Errors::ConfigError(e));
    }
    Ok(())
}

/**
 * Automatically detects the obsidian data path
 * TODO: Add Linux and maybe (very maybe) MacOS
 */
fn detect_obsidian_path() -> Option<String> {
    match std::env::consts::OS {
        "windows" => {
            let reader = fs::read_dir("~/AppData/Roaming/obsidian");
            match reader {
                Ok(_) => Some(String::from("~/AppData/Roaming/obsidian")),
                Err(_) => None,
            }
        }
        _ => {
            println!("Obsidian data path could not be detected");
            None
        }
    }
}

/**
 * This is the flow for setting up the obsidian data path
 */
fn setup_obsidian_path(config: &mut MyConfig) -> Result<(), errors::Errors> {
    // Check if the obsisiand path can be detected
    if let Some(path) = detect_obsidian_path() {
        config.obsidian_config = Some(PathBuf::from(path));
        match confy::store(APP_NAME, None, config) {
            Ok(_) => Ok(()),
            Err(e) => Err(errors::Errors::ConfigError(e)),
        }
    } else {
        println!("The obsidian data path could not be detected, please update it manually");
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Err(e) => Err(errors::Errors::IOError(e)),
            Ok(_) => {
                config.obsidian_config = Some(PathBuf::from(buffer));
                match confy::store(APP_NAME, None, config) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(errors::Errors::ConfigError(e))
                }
            }
        }
    }
}

/**
 * This is the flow for setting up the template directory
 */
fn setup_template_dir(config: &mut MyConfig) -> Result<(), errors::Errors> {
    println!("Please provide the absolute path to the template directory");
    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_line(&mut buffer) {
        return Err(errors::Errors::IOError(e));
    };
    config.template_path = Some(PathBuf::from(buffer));
    if let Err(e) = confy::store(APP_NAME, None, config) {
        return Err(errors::Errors::ConfigError(e));
    };
    Ok(())
}

/**
 * This is the function for the whole setup flow
 */
pub fn setup() -> Result<(), errors::Errors> {
    // Load the config, exit the programm if the config cannot be loaded
    let mut config: config::MyConfig = match confy::load(APP_NAME, None) {
        Err(e) => {
            println!("Could not load the config!: {:?}", e);
            exit(1);
        }
        Ok(c) => c,
    };
    // Setup the obsidian data path
    setup_obsidian_path(&mut config)?;
    // Setting up the vault dir
    setup_vault_dir(&mut config)?;
    // Setup template dir
    setup_template_dir(&mut config)?;

    Ok(())
}

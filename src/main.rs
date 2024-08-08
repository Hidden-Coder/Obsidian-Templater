use std::{path::PathBuf, process::exit, io};
use clap::{Parser, Subcommand};
use config::*;

use crate::obsidian::create_new_vault;

mod config;
mod helper;
mod obsidian;

const APP_NAME: &str = "ovt";

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// The name of the new vault to create
    #[arg(short='n', long="name")]
    vault_name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Edit the path for creating a new vault
    Vaults {
        //#[arg(short, long)]
        new_path: PathBuf
    },
    /// Edit the path for getting the template
    Template {
        //#[arg(short, long)]
        new_path: PathBuf
    },
    /// Set the obisian installation path
    Obsidian {
        new_path: PathBuf
    }
}

fn main() {
    let config: MyConfig = if let Ok(config) = confy::load(APP_NAME, None) {
        config
    } else {
        println!("Config was not found");
        return;
    };

    let args = Cli::parse();

    if args.vault_name.is_none() && args.command.is_none() {
        println!("You need to either specify the name of the vault to create or a subcommand. Try --help for more info");
    }
    match &args.command {
        Some(Commands::Vaults { new_path }) => {
            set_vault_dir(config, new_path.clone());
            println!("New vault dir has been set: {:?}", new_path);
        }
        Some(Commands::Template { new_path }) => {
            set_template_path(config, new_path.clone());
            println!("New template dir has been set: {:?}", new_path);
        }
        Some(Commands::Obsidian { new_path }) => {
            set_obsidian_path(config, new_path.clone());
            print!("Obsidian install path has been set: {:?}", new_path);

        }
        None => {
            let name = args.vault_name.expect("Vault name needs to be set");
            if let Err(e) = create_template(name.as_str(), config) {
                println!("An Error occured");
                println!("{:?}", e);
                exit(1);
            }
        }
    }
}

/**
 * Set the directory, that is used for creating new vault in.
 * This will be stored in the config file.
 */
fn set_vault_dir(mut config: MyConfig, new_path: PathBuf) {
    config.vault_dir = Some(new_path);
    if let Err(e) = confy::store(APP_NAME,None, config) {
        println!("Could not store into config");
        println!("{:?}", e);
        exit(1);
    }
}

/**
 * Set the template directory, that is used for getting the vault template from.
 * This will be stored in the config file.
 */
fn set_template_path(mut config: MyConfig, new_path: PathBuf) {
    config.template_path = Some(new_path);
    if let Err(e) = confy::store(APP_NAME,None, config) {
        println!("Could not store into config");
        println!("{:?}", e);
        exit(1);
    }
}

/**
 * Set the obsidian installation path.
 * This will be stored in the config file.
 */
fn set_obsidian_path(mut config: MyConfig, new_path: PathBuf) {
    config.obsidian_config = Some(new_path);
    if let Err(e) = confy::store(APP_NAME, None, config) {
        println!("Could not store into config");
        println!("{:?}", e);
        exit(1);
    }
}

/**
 * Create a new vault with the given name.
 * This will copy the template vault to the new vault directory.
 * Then opens obisian with the newly created vault.
 */
fn create_template(name: &str, cfg: MyConfig) -> io::Result<()> {
    let vault_path = if let Some(path) = &cfg.vault_dir {
        path
    } else {
        println!("No vault dir set");
        exit(1);
    };
    let template_path = if let Some(path) = &cfg.template_path {
        path
    } else {
        println!("No template dir set");
        exit(1);
    };
    let vault_dir = std::fs::read_dir(&vault_path)?;
    // Check if no other directory is already named like the given name
    for ele in vault_dir.into_iter() {
        let ele = match ele {
            Ok(dir) => dir,
            Err(e) => {
                println!("{:?}", e);
                exit(1);
            }
        };
        if ele.metadata()?.is_dir() && ele.file_name() == name {
            println!("There is already a directory with the name {name}");
            println!("Process has been canceled");
            exit(1);
        }
    }
    let _template_dir = std::fs::read_dir(&template_path)?;

    let new_vault_path = vault_path.join(name);

    helper::copy_dir_rec(&template_path, &new_vault_path)?;
    println!("A new vault has been created!");
    println!("Target Dir: {:?}", vault_path.join(name));
    println!("Template Dir: {:?}", template_path);

    create_new_vault(&cfg, &new_vault_path)?;

    let open_path = format!("obsidian://open?path={}", new_vault_path.to_str().unwrap());
    let open_path = open_path.replace(" ", "%20");

    std::process::Command::new("cmd")
        .args(["/C", "start", &open_path])
        .spawn()
        .expect("Could not start obsidian");

    return Ok(());
}

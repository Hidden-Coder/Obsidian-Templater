use std::{path::PathBuf, process::exit, io};
use clap::{Parser, Subcommand};
use config::*;

use crate::obsidian::create_new_vault;

mod config;
mod helper;
mod obsidian;
mod setup;
mod errors;

const APP_NAME: &str = "ovt";

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
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
    },
    New {
        /// The name of the new vault to create
        vault_name: Option<String>,
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

    if args.command.is_none() {
        println!("You need to either specify the name of the vault to create or a subcommand. Try --help for more info");
        exit(1);
    }
    match &args.command {
        Some(Commands::Vaults { new_path }) => {
            setup::set_vault_dir(config, new_path.clone());
            println!("New vault dir has been set: {:?}", new_path);
        }
        Some(Commands::Template { new_path }) => {
            setup::set_template_path(config, new_path.clone());
            println!("New template dir has been set: {:?}", new_path);
        }
        Some(Commands::Obsidian { new_path }) => {
            setup::set_obsidian_path(config, new_path.clone());
            print!("Obsidian install path has been set: {:?}", new_path);
        }
        Some(Commands::New { vault_name }) => {
            // Extract the new vault name
            let vaule_name = match vault_name.as_ref() {
                Some(name) => name,
                // If no vault name is given with the new subcommand, the user gave wrong input
                None => {
                    println!("You need to specify a vault name for the new vault");
                    exit(1);
                }
            };
            if let io::Result::Err(e) = create_template(vaule_name.as_str(), config) {
                println!("Something went wrong {:?}", e);
                exit(1);
            }
            println!("A new vault has been created");
        }
        None => {
            panic!("This code should never be reached");
        }
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

    Ok(())
}

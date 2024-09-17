use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use colored::*;
use paris::Logger;
use std::process::Command;
mod helpers;
use helpers::{helpers::*};

#[derive(Parser)]
#[command(name = "StarkNet CLI")]
#[command(about = "A CLI tool to simplify StarkNet development setup", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install {
        #[clap(long, action)]
        force: bool,
    },
    Init{
        //name of your workspace
        name: String,
        /// Clone a template repo from github (url)
        #[clap(short, long)]
        repo: Option<String>,
        /// Do not initialize a git repository
        #[clap(long)]
        no_git: bool,
        /// Initialize even if there are files
        #[clap(long, action)]
        force: bool,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { force } => {
            starknet_install(*force)?;
        }
        Commands::Init { name, repo, no_git, force } => {
            starknet_init(name, repo, *no_git, *force)?;
        }
    }

    Ok(())
}

fn starknet_install(force: bool) -> Result<()> {
    let mut log = Logger::new();

    if force {
        log.warn("Forcing installation, even if files are present.");
    }

    log.info("Welcome to StarkNet Dev Cli!");

    let scarb_version = prompt_user("Enter the scarb version (leave empty for latest):")?;
    let scarb_version = if scarb_version.trim().is_empty() {
        "latest".to_string()
    } else {
        scarb_version
    };

    install_asdf()?;
    install_scarb(&scarb_version)?;

    log.success("Installation completed successfully!");
    Ok(())
}


fn install_asdf() -> Result<()> {
    let mut log = Logger::new();

    let versioned_url = format!("https://github.com/asdf-vm/asdf.git",);
    if !is_installed("curl")? {
        install_package("curl").unwrap();
    }
    if !is_installed("git")? {
        install_package("git").unwrap();
    }

    if !is_installed("asdf").unwrap() {
        log.loading("Installing asdf...".cyan());
        let home_dir = std::env::var("HOME")?;
        let asdf_dir = format!("{}/.asdf", home_dir);
        let asdf_output = Command::new("git")
            .arg("clone")
            .arg(versioned_url.clone())
            .arg(&asdf_dir)
            .output()
            .unwrap();
        println!("asdf output: {:?}", asdf_output);
        update_bashrc()?;
        source_bashrc()?;
        log.success("asdf installed successfully.");
    } else {
        log.info("asdf is already installed".green());
    }

    Ok(())
}

fn install_scarb(version: &str) -> Result<()> {
    let mut log = Logger::new();
    log.loading(format!("Installing scarb version {}...", version.cyan()));
    if !is_installed("scarb").unwrap() {
        let plugin_output = Command::new("asdf")
            .arg("plugin")
            .arg("add")
            .arg("scarb")
            .output()
            .unwrap();

        if !plugin_output.status.success() {
            println!("plugin output: {:?}", plugin_output);
            let error = "Failed to add".to_string().red();
            return Err(anyhow!("{} {}", error, "scarb".cyan()));
        }
        let scarb_output = Command::new("asdf")
            .arg("install")
            .arg("scarb")
            .arg(version)
            .output()
            .unwrap();

        if scarb_output.status.success() {
            log.success("scarb installed successfully.");
        } else {
            println!("scarb output: {:?}", scarb_output);
            let error = "Failed to install".to_string().red();
            return Err(anyhow!("{} {}", error, "scarb".cyan()));
        }
    } else {
        log.info("scarb is already installed".green());
    }

    Ok(())
}

fn starknet_init(name: &str, repo: &Option<String>, no_git: bool, force: bool) -> Result<()> { todo!()}
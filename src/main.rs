use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use colored::*;
use paris::Logger;
use reqwest;
use serde_json;
use std::io::{self, Write};
use std::process::Command;

#[derive(serde::Deserialize)]
struct ReleaseInfo {
    tag_name: String,
}

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
        /// Install the starknet dev tools
        #[clap(long, action)]
        force: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { force } => {
            starknet_install(*force)?;
        }
    }

    Ok(())
}

fn starknet_install(force: bool) -> Result<()> {
    let mut log = Logger::new();

    if force {
        log.warn("Forcing installation, even if files are present.");
    }

    log.info("Starting the installation process for StarkNet.");

    let asdf_version = prompt_user("Enter the asdf version (leave empty for latest):")?;
    let asdf_version = if asdf_version.trim().is_empty() {
        "latest".to_string()
    } else {
        asdf_version
    };

    let scarb_version = prompt_user("Enter the scarb version (leave empty for latest):")?;
    let scarb_version = if scarb_version.trim().is_empty() {
        "latest".to_string()
    } else {
        scarb_version
    };

    log.info(format!(
        "Installing asdf version {} and scarb version {}...",
        asdf_version.cyan(),
        scarb_version.cyan()
    ));

    install_asdf(&asdf_version)?;
    install_scarb(&scarb_version)?;

    log.success("Installation completed successfully!");

    Ok(())
}

// Function to prompt the user for input
fn prompt_user(prompt: &str) -> Result<String> {
    print!("{} ", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

// Simulate asdf installation process
fn install_asdf(version: &str) -> Result<()> {
    let mut log = Logger::new();
    log.log("lets goo");
    if !is_installed("curl")? {
        println!("inside curl");
        install_package("curl").unwrap();
    }
    println!("didnt go inside curl");
    if !is_installed("git")? {
        install_package("git").unwrap();
    }
    println!("didnt go inside git");
    if !is_installed("asdf").unwrap() {
        let latest_version = if version != "latest" {
            version.to_string()
        } else {
            get_latest_version("asdf-vm", "asdf").unwrap()
        };

        let versioned_url = format!(
            "https://github.com/asdf-vm/asdf.git ~/.asdf --branch {}",
            latest_version
        );
        Command::new("git")
            .arg("clone")
            .arg(versioned_url)
            .output()
            .unwrap();
    }

    log.info(format!("Installing asdf version {}...", version.cyan()));
    log.success("asdf installed successfully.");

    Ok(())
}

// Simulate scarb installation process
fn install_scarb(version: &str) -> Result<()> {
    let mut log = Logger::new();

    // Simulate checking if version exists
    if version == "notfound" {
        return Err(anyhow!("scarb version {} not found.", version));
    }

    log.info(format!("Installing scarb version {}...", version.cyan()));
    // Simulate success
    log.success("scarb installed successfully.");

    Ok(())
}

fn is_installed(package: &str) -> Result<bool> {
    let output = Command::new("which").arg(package).output()?;
    println!("is_installed called for {}", package);
    println!("{:?}", output);
    Ok(output.status.success())
}

fn install_package(package: &str) -> Result<()> {
    let mut log = Logger::new();
    let apt_output = Command::new("sudo")
        .arg("apt")
        .arg("install")
        .arg("-y")
        .arg(package)
        .output()?;

    if !apt_output.status.success() {
        let error = "Failed to install {}".to_string().red();
        return Err(anyhow!("{} {}", error, package.cyan()));
    }
    let success = "{} installed successfully.".to_string().green();
    log.success(format!("{} {}", success, package.cyan()));

    Ok(())
}

fn get_latest_version(package: &str, owner: &str) -> Result<String> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        owner, package
    );
    let output = reqwest::blocking::get(url)?.text()?;
    let release_info: ReleaseInfo = serde_json::from_str(&output)?;
    Ok(release_info.tag_name)
}

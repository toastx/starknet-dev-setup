use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use colored::*;
use paris::Logger;
use std::process::Command;
mod helpers;
use helpers::helpers::*;

#[derive(Parser)]
#[command(name = "Starknet Dev Setup")]
#[command(about = "A downloader to simplify StarkNet development setup", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install {
        /// Initialize even if there are files
        #[clap(long, action)]
        force: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut log = Logger::new();
    match &cli.command {
        Commands::Install { force } => {
            starknet_install(*force, &mut log)?;
        }
    }
    log.success("The dev setup is completed successfully!");
    log.info(format!(
        "use {} {} to create a new project",
        "scarb new".cyan(),
        "<project_name>".green()
    ));

    Ok(())
}

fn starknet_install(force: bool, log: &mut Logger) -> Result<()> {
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

    install_asdf(log)?;
    install_scarb(&scarb_version, log)?;
    install_snfoundry(log)?;

    log.success("Installation completed successfully!");
    Ok(())
}

fn install_asdf(log: &mut Logger) -> Result<()> {
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
        if asdf_output.status.success() {
            log.log(format!("{:?}", asdf_output));
            log.success("asdf installed successfully.");
            let choice = prompt_user("Do you want to automate .bashrc edits (y/n):")?;
            let choice = if choice.trim() == "y" { 1 } else { 0 };
            if choice == 1 {
                update_bashrc()?;
                source_bashrc()?;

                log.log(format!(
                    "Please run {} again to initialise asdf",
                    "starkdev install".cyan()
                ));
            } else {
                log.info("add the following to your bashrc file");
                log.log(". $HOME/.asdf/asdf.sh");
                log.log(". $HOME/.asdf/completions/asdf.bash");
                log.log("and rerun the cli");
                std::process::exit(0);
            }
        } else {
            let error = String::from_utf8_lossy(&asdf_output.stderr)
                .into_owned()
                .to_string()
                .red();
            return Err(anyhow!("{}", error));
        }
    } else {
        log.info("asdf is already installed".green());
    }

    Ok(())
}

fn install_scarb(version: &str, log: &mut Logger) -> Result<()> {
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
            let error = String::from_utf8_lossy(&plugin_output.stderr)
                .into_owned()
                .to_string()
                .red();
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
            let error = String::from_utf8_lossy(&scarb_output.stderr)
                .into_owned()
                .to_string()
                .red();
            return Err(anyhow!("{} {}", error, "scarb".cyan()));
        }

        let _add_scarb_global = Command::new("asdf")
            .arg("global")
            .arg("scarb")
            .arg("latest")
            .output()
            .unwrap();
    } else {
        log.info("scarb is already installed".green());
    }

    Ok(())
}
fn install_snfoundry(log: &mut Logger) -> Result<()> {
    log.loading("Installing snforge...".cyan());
    let snforge_plugin_output = Command::new("asdf")
        .arg("plugin")
        .arg("add")
        .arg("starknet-foundry")
        .output()
        .unwrap();

    if !snforge_plugin_output.status.success() {
        println!("plugin output: {:?}", snforge_plugin_output);
        let error = String::from_utf8_lossy(&snforge_plugin_output.stderr)
            .into_owned()
            .to_string()
            .red();
        return Err(anyhow!("{} {}", error, "scarb".cyan()));
    }

    let snforge_output = Command::new("asdf")
        .arg("install")
        .arg("starknet-foundry")
        .arg("latest")
        .output()
        .unwrap();

    if snforge_output.status.success() {
        log.success("snforge installed successfully.");
    } else {
        let error = String::from_utf8_lossy(&snforge_output.stderr)
            .into_owned()
            .to_string()
            .red();
        return Err(anyhow!("{} {}", error, "snforge".cyan()));
    }

    Ok(())
}

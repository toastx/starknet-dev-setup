use anyhow::{anyhow, Result};
use colored::*;
use paris::Logger;
use std::fs::{read_to_string, OpenOptions};
use std::io::{self, Write};
use std::process::Command;


pub fn update_bashrc() -> Result<()> {
    let mut log = Logger::new();
    let bashrc_path = std::env::var("HOME").unwrap() + "/.bashrc";
    println!("bashrc path: {}", bashrc_path);
    let bashrc_content = read_to_string(&bashrc_path)?;
    let asdf_lines = r#"
# Load asdf completions and asdf itself
. "$HOME/.asdf/asdf.sh"
. "$HOME/.asdf/completions/asdf.bash"

"#;

    if !bashrc_content.contains(".asdf/asdf.sh") {
        let mut file = OpenOptions::new().append(true).open(&bashrc_path)?;
        file.write_all(asdf_lines.as_bytes())?;
    } else {
        log.warn(".bashrc is already configured for asdf.");
    }
    Ok(())
}


pub fn source_bashrc() -> io::Result<()> {
    let status = Command::new("bash")
        .arg("-c")
        .arg("source ~/.bashrc")
        .status()?;

    if status.success() {
        println!("Successfully sourced ~/.bashrc.");
    } else {
        println!("Failed to source ~/.bashrc. Please restart your terminal.");
    }

    Ok(())
}

pub fn prompt_user(prompt: &str) -> Result<String> {
    print!("{} ", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

pub fn is_installed(package: &str) -> Result<bool> {
    let output = Command::new("which").arg(package).output()?;
    if output.status.success() {
        println!("{} is installed", package.cyan());
    }
    Ok(output.status.success())
}

pub fn install_package(package: &str) -> Result<()> {
    let mut log = Logger::new();
    let apt_output = Command::new("sudo")
        .arg("apt")
        .arg("install")
        .arg("-y")
        .arg(package)
        .output()?;

    if !apt_output.status.success() {
        let error = "Failed to install".to_string().red();
        return Err(anyhow!("{} {}", error, package.cyan()));
    }
    let success = "installed successfully.".to_string().green();
    log.success(format!("{} {}", package.cyan(), success));

    Ok(())
}

use serde::{Deserialize, Serialize};
use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Deserialize, Serialize)]
pub struct Symlink {
    pub source: String,
    pub destination: String,
    pub r#type: String, // use r#type because "type" is a Rust keyword
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub sort_by: String,
    pub install_command: String,
    pub symlinks: Vec<Symlink>,
    pub dotfiles_dir: String,
    pub scripts_dir: String,
}

pub fn load_app_config(filename: &str) -> Result<AppConfig, String> {
    let content = std::fs::read_to_string(filename).map_err(|e| e.to_string())?;
    toml::from_str(&content).map_err(|e| e.to_string())
}

pub fn run_symlinks(app_config: &AppConfig) -> Result<(), String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let dotfiles_dir = Path::new(&app_config.dotfiles_dir);

    for link in &app_config.symlinks {
        let source = dotfiles_dir.join(&link.source);
        let destination = home_dir.join(&link.destination);

        if destination.exists() {
            eprintln!(
                "Destination already exists: {}. Skipping symlink for {}. Exiting.",
                destination.display(),
                link.destination
            );
            continue;
        }

        println!(
            "Symlinking {} to {}",
            source.display(),
            destination.display()
        );
        unix_fs::symlink(&source, &destination)
            .map_err(|e| format!("Failed to create symlink for {}: {}", link.destination, e))?;
    }
    Ok(())
}

pub fn list_scripts(app_config: &AppConfig) -> Result<(), String> {
    let scripts_path = &app_config.scripts_dir;
    let entries = fs::read_dir(scripts_path)
        .map_err(|e| format!("Failed to read scripts directory: {}", e))?;

    let mut script_names = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                script_names.push(name.to_string());
            }
        }
    }

    script_names.sort();

    for name in script_names {
        println!("{name}");
    }
    Ok(())
}

pub fn run_script(app_config: &AppConfig, script_name: &str) -> Result<(), String> {
    let script_path = Path::new(&app_config.scripts_dir).join(script_name);

    if !script_path.exists() {
        return Err(format!("Script not found: {}", script_path.display()));
    }
    if !script_path.is_file() {
        return Err(format!("Not a file: {}", script_path.display()));
    }

    // Run the script (assumes it is already executable and has a shebang)
    let status = Command::new(&script_path)
        .status()
        .map_err(|e| format!("Failed to run script: {}", e))?;

    if status.success() {
        println!("Script '{}' ran successfully.", script_name);
        Ok(())
    } else {
        Err(format!(
            "Script '{}' exited with status: {}",
            script_name, status
        ))
    }
}

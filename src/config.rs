use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    pub reason: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub packages: Vec<Package>,
}

pub fn install(package: &str, config: &mut Config) -> Result<(), String> {
    // Check if the package is already in the config, skip installation if it is
    if config.packages.iter().any(|p| p.name == package) {
        println!("Package {} is already installed, skipping.", package);
        return Ok(());
    }

    // Add the package to the config if it is not already present
    config.packages.push(Package {
        name: package.to_string(),
        reason: "ADD DETAILS HERE".to_string(),
        tags: vec!["ADD TAGS HERE".to_string()],
    });

    // Simulate installation logic
    println!("Installing package: {}", package);
    Ok(())
}

pub fn uninstall(package: &str, config: &mut Config) -> Result<(), String> {
    // Check if the package exists in the config
    if let Some(pos) = config.packages.iter().position(|p| p.name == package) {
        // Remove the package from the config
        config.packages.remove(pos);
        println!("Uninstalled package: {}", package);
        Ok(())
    } else {
        Err(format!("Package {} not found in configuration.", package))
    }
}

pub fn save_config(config: &Config, filename: &str) -> Result<(), String> {
    let toml_string = toml::to_string(config).map_err(|e| e.to_string())?;
    std::fs::write(filename, toml_string).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_packages(filename: &str) -> Result<Config, String> {
    let content = std::fs::read_to_string(filename).map_err(|e| e.to_string())?;
    toml::from_str(&content).map_err(|e| e.to_string())
}

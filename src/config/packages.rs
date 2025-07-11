use crate::config::AppConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    pub reason: String,
    pub category: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Packages {
    pub packages: Vec<Package>,
}

pub fn install(package: &str, config: &mut Packages) -> Result<(), String> {
    // Check if the package is already in the config, skip installation if it is
    if config.packages.iter().any(|p| p.name == package) {
        println!("Package {} is already installed, skipping.", package);
        return Ok(());
    }

    // Add the package to the config if it is not already present
    config.packages.push(Package {
        name: package.to_string(),
        reason: "ADD DETAILS HERE".to_string(),
        category: "ADD CATEGORY HERE".to_string(),
        tags: vec!["ADD TAGS HERE".to_string()],
    });

    // Simulate installation logic
    println!("Installing package: {}", package);
    println!("Don't forget to add details, category, and tags for the package.");
    Ok(())
}

pub fn uninstall(package: &str, config: &mut Packages) -> Result<(), String> {
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

pub fn save_packages(
    packages: &mut Packages,
    config: &AppConfig,
    filename: &str,
) -> Result<(), String> {
    let toml_string;

    for pkg in &mut packages.packages {
        pkg.tags.sort();
    }

    match config.sort_by.as_str() {
        "package_name" => {
            packages.packages.sort_by(|a, b| a.name.cmp(&b.name));
        }
        "category" => {
            packages
                .packages
                .sort_by(|a, b| match a.category.cmp(&b.category) {
                    std::cmp::Ordering::Equal => a.name.cmp(&b.name),
                    other => other,
                });
        }
        _ => {}
    }

    if config.pretty_print {
        toml_string = toml::to_string_pretty(packages).map_err(|e| e.to_string())?;
    } else {
        toml_string = toml::to_string(packages).map_err(|e| e.to_string())?;
    }

    std::fs::write(filename, toml_string).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_packages(filename: &str) -> Result<Packages, String> {
    let content = std::fs::read_to_string(filename).map_err(|e| e.to_string())?;
    toml::from_str(&content).map_err(|e| e.to_string())
}

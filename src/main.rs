use std::fs;

mod config;

fn main() {
    const FILENAME: &str = "packages.toml";

    let mut config: config::Config = match fs::read_to_string(FILENAME) {
        Ok(content) => toml::from_str(&content).expect("Failed to parse TOML"),
        Err(e) => {
            eprintln!("Error reading file {}: {}", FILENAME, e);
            return;
        }
    };

    for pkg in &config.packages {
        println!("Package Name: {}", pkg.name);
        println!("Reason: {}", pkg.reason);
        println!("Tags: {:?}", pkg.tags);
        println!();
    }

    config::install("kitty", &mut config).expect("Failed to install package");
    config::save_config(&config, FILENAME).expect("Failed to save config");
    // config::uninstall("kitty", &mut config).expect("Failed to uninstall package");
    // config::save_config(&config, FILENAME).expect("Failed to save config after uninstall");
}

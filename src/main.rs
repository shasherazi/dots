use clap::{Parser, Subcommand};

mod config;

#[derive(Parser)]
#[command(name = "dots")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a package
    Install { package: String },
    /// Uninstall a package
    Uninstall { package: String },
}

fn main() {
    const FILENAME: &str = "packages.toml";
    let cli = Cli::parse();

    let mut packages =
        config::load_packages("packages.toml").expect("Failed to load packages.toml");
    let app_config = config::load_app_config("config.toml").expect("Failed to load config.toml");

    match &cli.command {
        Commands::Install { package } => {
            if let Err(e) = config::install(package, &mut packages) {
                eprintln!("Error installing package {}: {}", package, e);
            } else {
                if let Err(e) = config::save_packages(&mut packages, &app_config, FILENAME) {
                    eprintln!("Error saving configuration: {}", e);
                }
            }
        }
        Commands::Uninstall { package } => {
            if let Err(e) = config::uninstall(package, &mut packages) {
                eprintln!("Error uninstalling package {}: {}", package, e);
            } else {
                if let Err(e) = config::save_packages(&mut packages, &app_config, FILENAME) {
                    eprintln!("Error saving configuration: {}", e);
                }
            }
        }
    }

    for pkg in &packages.packages {
        println!("Package Name: {}", pkg.name);
        println!("Reason: {}", pkg.reason);
        println!("Tags: {:?}", pkg.tags);
        println!();
    }
}

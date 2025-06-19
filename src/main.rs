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

    let mut config = config::load_packages("packages.toml").expect("Failed to load packages.toml");

    match &cli.command {
        Commands::Install { package } => {
            if let Err(e) = config::install(package, &mut config) {
                eprintln!("Error installing package {}: {}", package, e);
            } else {
                if let Err(e) = config::save_config(&config, FILENAME) {
                    eprintln!("Error saving configuration: {}", e);
                }
            }
        }
        Commands::Uninstall { package } => {
            if let Err(e) = config::uninstall(package, &mut config) {
                eprintln!("Error uninstalling package {}: {}", package, e);
            } else {
                if let Err(e) = config::save_config(&config, FILENAME) {
                    eprintln!("Error saving configuration: {}", e);
                }
            }
        }
    }

    for pkg in &config.packages {
        println!("Package Name: {}", pkg.name);
        println!("Reason: {}", pkg.reason);
        println!("Tags: {:?}", pkg.tags);
        println!();
    }
}

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
    Install {
        package: String,
    },
    Uninstall {
        package: String,
    },
    List {
        #[arg(long)]
        tag: Option<String>,
    },
    Info {
        package: String,
    },
    Edit {
        package: String,
        #[arg(long)]
        reason: Option<String>,
        #[arg(long)]
        category: Option<String>,
        #[arg(long)]
        tags: Option<String>,
    },
    Has {
        package: String,
    },
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
        Commands::List { tag } => {
            if let Some(tag) = tag {
                let filtered_packages: Vec<_> = packages
                    .packages
                    .iter()
                    .filter(|p| p.tags.contains(&tag.to_string()))
                    .collect();
                for pkg in filtered_packages {
                    println!("Package Name: {}", pkg.name);
                    println!("Reason: {}", pkg.reason);
                    println!("Tags: {:?}", pkg.tags);
                    println!();
                }
            } else {
                for pkg in &packages.packages {
                    println!("Package Name: {}", pkg.name);
                    println!("Reason: {}", pkg.reason);
                    println!("Tags: {:?}", pkg.tags);
                    println!();
                }
            }
        }
        Commands::Info { package } => {
            if let Some(pkg) = packages.packages.iter().find(|p| p.name == *package) {
                println!("Package Name: {}", pkg.name);
                println!("Reason: {}", pkg.reason);
                println!("Tags: {:?}", pkg.tags);
            } else {
                eprintln!("Package {} not found.", package);
            }
        }
        Commands::Edit {
            package,
            reason,
            category,
            tags,
        } => {
            if let Some(pkg) = packages.packages.iter_mut().find(|p| p.name == *package) {
                if let Some(reason) = reason {
                    pkg.reason = reason.clone();
                }
                if let Some(tags) = tags {
                    pkg.tags = tags.split(',').map(|s| s.trim().to_string()).collect();
                }
                if let Some(category) = category {
                    pkg.category = category.clone();
                }
                if let Err(e) = config::save_packages(&mut packages, &app_config, FILENAME) {
                    eprintln!("Error saving configuration: {}", e);
                }
            } else {
                eprintln!("Package {} not found.", package);
            }
        }
        Commands::Has { package } => {
            if packages.packages.iter().any(|p| p.name == *package) {
                println!("Package {} is installed.", package);
            } else {
                println!("Package {} is not installed.", package);
            }
        }
    }

    for pkg in &packages.packages {
        println!("Package Name: {}", pkg.name);
        println!("Reason: {}", pkg.reason);
        println!("Category: {}", pkg.category);
        println!("Tags: {:?}", pkg.tags);
        println!();
    }
}

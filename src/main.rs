use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

mod config;

#[derive(Parser)]
#[command(name = "dots")]
struct Cli {
    #[arg(short, long, global = true)]
    config_dir: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install {
        package: String,
    },
    InstallAll,
    Uninstall {
        package: String,
    },
    List {
        #[arg(long)]
        tag: Option<String>,
        #[arg(long)]
        category: Option<String>,
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
    Run {
        script_name: String,
    },
    Scripts,
    Symlink,
}

fn resolve_config_dir(cli_config_dir: &Option<String>) -> PathBuf {
    if let Some(dir) = cli_config_dir {
        return PathBuf::from(dir);
    }
    if let Some(xdg_config_home) = std::env::var_os("XDG_CONFIG_HOME") {
        let xdg_path = Path::new(&xdg_config_home).join("dots");
        if xdg_path.exists() {
            return xdg_path;
        }
    }
    if let Some(home) = dirs::home_dir() {
        let home_path = home.join(".config/dots");
        if home_path.exists() {
            return home_path;
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn main() {
    let cli = Cli::parse();

    let config_dir = resolve_config_dir(&cli.config_dir);
    let config_path = config_dir.join("config.toml");
    let packages_path = config_dir.join("packages.toml");

    let mut packages = config::load_packages(packages_path.to_str().unwrap())
        .expect("Failed to load packages.toml");
    let app_config =
        config::load_app_config(config_path.to_str().unwrap()).expect("Failed to load config.toml");

    match &cli.command {
        Commands::Install { package } => {
            if let Err(e) = config::install(package, &mut packages) {
                eprintln!("Error installing package {}: {}", package, e);
            } else {
                if let Err(e) = config::save_packages(
                    &mut packages,
                    &app_config,
                    packages_path.to_str().unwrap(),
                ) {
                    eprintln!("Error saving configuration: {}", e);
                }
            }
        }
        Commands::InstallAll => {
            let package_names: Vec<String> =
                packages.packages.iter().map(|p| p.name.clone()).collect();
            let joined = package_names.join(" ");

            let command_template = &app_config.install_command;
            let command_str = command_template.replace("{packages}", &joined);

            let mut parts = command_str.split_whitespace();
            if let Some(program) = parts.next() {
                let args: Vec<&str> = parts.collect();
                println!("Running: {} {:?}", program, args);

                let status = std::process::Command::new(program).args(&args).status();

                match status {
                    Ok(status) if status.success() => {
                        println!("Install command completed successfully.");
                    }
                    Ok(status) => {
                        eprintln!("Install command failed with status: {}", status);
                    }
                    Err(e) => {
                        eprintln!("Failed to run install command: {}", e);
                    }
                }
            } else {
                eprintln!("Invalid install command in config.");
            }
        }
        Commands::Uninstall { package } => {
            if let Err(e) = config::uninstall(package, &mut packages) {
                eprintln!("Error uninstalling package {}: {}", package, e);
            } else {
                if let Err(e) = config::save_packages(
                    &mut packages,
                    &app_config,
                    packages_path.to_str().unwrap(),
                ) {
                    eprintln!("Error saving configuration: {}", e);
                }
            }
        }
        Commands::List { tag, category } => {
            if let Some(tag) = tag {
                let filtered_packages: Vec<_> = packages
                    .packages
                    .iter()
                    .filter(|p| p.tags.contains(&tag.to_string()))
                    .collect();
                for pkg in filtered_packages {
                    println!("Package Name: {}", pkg.name);
                    println!("Reason: {}", pkg.reason);
                    println!("Category: {}", pkg.category);
                    println!("Tags: {:?}", pkg.tags);
                    println!();
                }
            } else if let Some(category) = category {
                let filtered_packages: Vec<_> = packages
                    .packages
                    .iter()
                    .filter(|p| p.category == *category)
                    .collect();
                for pkg in filtered_packages {
                    println!("Package Name: {}", pkg.name);
                    println!("Reason: {}", pkg.reason);
                    println!("Category: {}", pkg.category);
                    println!("Tags: {:?}", pkg.tags);
                    println!();
                }
            } else {
                for pkg in &packages.packages {
                    println!("Package Name: {}", pkg.name);
                    println!("Reason: {}", pkg.reason);
                    println!("Category: {}", pkg.category);
                    println!("Tags: {:?}", pkg.tags);
                    println!();
                }
            }
        }
        Commands::Info { package } => {
            if let Some(pkg) = packages.packages.iter().find(|p| p.name == *package) {
                println!("Package Name: {}", pkg.name);
                println!("Reason: {}", pkg.reason);
                println!("Category: {}", pkg.category);
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
                if let Err(e) = config::save_packages(
                    &mut packages,
                    &app_config,
                    packages_path.to_str().unwrap(),
                ) {
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
        Commands::Run { script_name } => {
            if let Err(e) = config::run_script(&app_config, script_name) {
                eprintln!("Error running script: {}", e);
            }
        }
        Commands::Symlink => {
            if let Err(e) = config::run_symlinks(&app_config) {
                eprintln!("Symlink error: {}", e);
            }
        }
        Commands::Scripts => {
            if let Err(e) = config::list_scripts(&app_config) {
                eprintln!("Error listing scripts: {}", e);
            }
        }
    }
}

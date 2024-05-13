mod cli;
mod constants;
mod os;
mod traits;
mod utils;

use anyhow::bail;
use clap::Parser;
use os::Arch;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::exit;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use traits::OperatingSystem;

use crate::cli::{App, Command};

#[derive(Debug, Deserialize)]
struct System {
    packages: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Home {
    groups: Vec<String>,
    #[serde(rename = "default-shell")]
    default_shell: String,
    links: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct Data {
    system: System,
    users: HashMap<String, Home>,
    //programs: HashMap<String, bool>,
}

fn main() -> anyhow::Result<()> {
    let args = App::parse();

    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Linux system manager");

    let os = os_info::get();
    info!("Detected OS: {:?}", os.os_type());
    info!("Architecture: {:?}", os.architecture());

    let handler: Box<dyn OperatingSystem> = match os.os_type() {
        os_info::Type::Arch => Box::new(Arch::init()?),
        _ => unimplemented!(),
    };

    match args.command {
        Command::Sync {
            config_path,
            system,
            home,
        } => {
            let contents = match fs::read_to_string(config_path.clone()) {
                Ok(c) => c,
                Err(_) => {
                    eprintln!("Could not read file `{}`", config_path.display());
                    exit(1);
                }
            };
            let data: Data = match serde_yaml::from_str(&contents) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Unable to load data from `{}` {}", config_path.display(), e);
                    exit(1);
                }
            };

            if !system && !home {
                bail!("Neither system sync nor home sync are selected");
            }

            if system {
                sync_system(&handler, data.system)?;
            }

            if home {
                sync_home(&handler, data.users)?;
            }
        }
    }

    Ok(())
}

fn sync_system(os: &Box<dyn OperatingSystem>, system_config: System) -> anyhow::Result<()> {
    let exit_status = os.install_packages(system_config.packages)?;
    if !exit_status.success() {
        bail!("Could not install system packages");
    }
    Ok(())
}

fn sync_home(
    os: &Box<dyn OperatingSystem>,
    users_config: HashMap<String, Home>,
) -> anyhow::Result<()> {
    for (username, home_config) in users_config {
        // Ensure user exists, otherwise create it.
        if os.user_exists(&username)? {
            // Make sure groups exist.
            os.user_update(&username, home_config.groups, home_config.default_shell)?;
        } else {
            // Create user with groups & default shell
            os.user_create(&username, home_config.groups, home_config.default_shell)?;
        }

        // Check links
        // TODO: Do we need a lock file to compare old / new & remove old links?
        println!("Links: {:?}", home_config.links);
    }

    Ok(())
}

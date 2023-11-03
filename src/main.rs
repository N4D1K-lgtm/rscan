use clap::{ArgGroup, Parser};
use std::error::Error;

mod modules;
mod styles;

use modules::ModuleManager;
pub use styles::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("module").args(&["gateway", "dns", "arp", "lldp", "all"]).required(false)))]
struct Cli {
    /// get default gateway IP address(s)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    gateway: bool,

    /// get DNS server(s)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    dns: bool,

    /// get ARP/NDP table
    #[arg(long, action = clap::ArgAction::SetTrue)]
    arp: bool,

    /// get LLDP neighbors
    #[arg(long, action = clap::ArgAction::SetTrue)]
    lldp: bool,

    /// run every module
    #[arg(long, action = clap::ArgAction::SetTrue)]
    all: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let module_manager = ModuleManager::new();

    let all_module_names = module_manager.get_module_names();

    // Determine which modules to run based on the CLI input
    let mut modules_to_run = vec![];
    if cli.all {
        modules_to_run.extend(all_module_names);
    } else {
        if cli.gateway {
            modules_to_run.push("gateway".to_string());
        }
        if cli.dns {
            modules_to_run.push("dns".to_string());
        }
        if cli.arp {
            modules_to_run.push("arp".to_string());
        }
        if cli.lldp {
            modules_to_run.push("lldp".to_string());
        }

        // Default to all modules if none are specified
        if modules_to_run.is_empty() {
            modules_to_run.extend(all_module_names);
        }
    }

    let tables = module_manager.run_selected_modules(&modules_to_run).await?;

    ModuleManager::print_tables(tables);

    Ok(())
}

// Implement the trait for your actual modules...

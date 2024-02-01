use clap::{ArgAction, ArgGroup, Parser};

#[derive(Parser)]
#[command(group(ArgGroup::new("module").args(&["gateway", "dns", "arp", "lldp", "all"]).required(false)))]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// get default gateway IP address(s)
    #[arg(long, action = ArgAction::SetTrue)]
    gateway: bool,

    /// get DNS server(s)
    #[arg(long, action = ArgAction::SetTrue)]
    dns: bool,

    /// get ARP/NDP table
    #[arg(long, action = ArgAction::SetTrue)]
    arp: bool,

    /// get LLDP neighbors
    #[arg(long, action = ArgAction::SetTrue)]
    lldp: bool,

    /// run every module
    #[arg(long, action = ArgAction::SetTrue)]
    all: bool,
}

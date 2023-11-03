use prettytable::*;
use std::process::{Command, Output};
use std::str;

#[derive(Debug, Clone)]
pub struct ArpEntry {
    pub ip: String,
    pub hw_type: String,
    pub flags: String,
    pub hw_address: String,
    pub mask: String,
    pub device: String,
}

const COLOR_SUCCESS: color::Color = color::BLUE;
const COLOR_WARNING: color::Color = color::YELLOW;
const COLOR_ERROR: color::Color = color::RED;
const COLOR_INFO: color::Color = color::GREEN;
const COLOR_HIGHLIGHT: color::Color = color::CYAN;

// Helper function to create a cell with specific foreground color
fn create_colored_cell(content: &str, fg_color: color::Color) -> Cell {
    Cell::new(content).with_style(Attr::ForegroundColor(fg_color))
}

// Helper function to create a bold cell
fn create_bold_cell(content: &str) -> Cell {
    Cell::new(content).with_style(Attr::Bold)
}

fn main() {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_DEFAULT);

    // Style header cells to be bold and different color
    let header_cell_style = |content: &str| -> Cell {
        create_colored_cell(content, color::BRIGHT_BLUE).with_style(Attr::Bold)
    };

    // Adding Default Gateway
    let default_gateway = get_default_gateway();
    table.add_row(Row::new(vec![
        header_cell_style("Default Gateway"),
        create_colored_cell(&default_gateway, COLOR_INFO),
    ]));

    // Adding DNS Servers
    let dns_servers = get_dns_servers();
    table.add_row(Row::new(vec![
        header_cell_style("DNS Servers"),
        create_colored_cell(&dns_servers.join("\n"), COLOR_INFO),
    ]));

    // Get VLAN Interfaces
    let vlan_interfaces = get_vlan_interfaces();
    let vlan_interfaces_formatted = vlan_interfaces
        .into_iter()
        .map(|interface| {
            let title_cell = header_cell_style("VLAN Interfaces");
            let interface_cell = if interface == "No VLAN interfaces found" {
                create_colored_cell(&interface, COLOR_ERROR) // Assuming you have a COLOR_WARNING for this kind of message
            } else {
                create_colored_cell(&interface, COLOR_INFO)
            };
            Row::new(vec![title_cell, interface_cell])
        })
        .collect::<Vec<Row>>();

    // Now add these rows to your table
    for row in vlan_interfaces_formatted {
        table.add_row(row);
    }

    // Adding ARP Entries
    let arp_entries = get_arp_entries();
    let arp_entries_str = match arp_entries.clone() {
        Ok(entries) => entries
            .iter()
            .map(|entry| {
                format!(
                    "{:15} {:17} {:5} {:17} {:15} {}",
                    entry.ip,
                    entry.hw_type,
                    entry.flags,
                    entry.hw_address,
                    entry.mask,
                    entry.device
                )
            })
            .collect::<Vec<String>>()
            .join("\n"),
        Err(e) => e.to_string(),
    };

    table.add_row(Row::new(vec![
        header_cell_style("ARP Entries"),
        create_colored_cell(&arp_entries_str, COLOR_INFO),
    ]));

    // Add a header row for Reverse DNS Lookups
    table.add_row(row![
        header_cell_style("Reverse DNS Lookups"),
        "IP Address".to_string(),
        "Resolved Hostname".to_string()
    ]);

    let reverse_dns_lookups = match arp_entries {
        Ok(entries) => get_reverse_dns_for_arp(&entries),
        Err(_) => vec![(
            "".to_string(),
            "".to_string(),
            "No ARP entries found for reverse DNS lookup.".to_string(),
        )],
    };
    let reverse_dns_formatted = reverse_dns_lookups
        .into_iter()
        .map(|(ip, hostname, status_msg)| {
            let ip_cell = create_colored_cell(&ip, color::BRIGHT_BLUE).with_style(Attr::Bold);
            let hostname_cell = if hostname == "N/A" {
                create_colored_cell(&hostname, COLOR_ERROR)
            } else {
                create_colored_cell(&hostname, COLOR_SUCCESS)
            };
            let status_cell = create_colored_cell(&status_msg, COLOR_INFO);
            Row::new(vec![ip_cell, hostname_cell, status_cell])
        })
        .collect::<Vec<Row>>();

    // Now add these rows to your table
    for row in reverse_dns_formatted {
        table.add_row(row);
    }

    // adding interface configurations
    let interface_configs = get_interface_configs();
    table.add_row(row![
        "Interface Configurations",
        interface_configs.join("\n")
    ]);

    // adding routing table
    let routing_table = get_routing_table();
    table.add_row(row!["Routing Table", routing_table.join("\n")]);

    // adding listening ports
    let listening_ports = get_listening_ports();
    table.add_row(row!["Listening Ports", listening_ports.join("\n")]);

    // adding LLDP neighbors, if available
    let lldp_neighbors = get_lldp_neighbors();
    table.add_row(row!["LLDP Neighbors", lldp_neighbors]);

    // performing traceroute to first DNS server
    let traceroute_output = perform_traceroute_to_first_dns(&dns_servers);
    table.add_row(row!["Traceroute to First DNS", traceroute_output]);

    // check common services on the default gateway
    let common_services = check_common_services(&default_gateway);
    table.add_row(row![
        header_cell_style("Common Services on Gateway"),
        common_services
            .iter()
            .map(|service| if service.contains("open") {
                create_colored_cell(service, COLOR_SUCCESS).to_string()
            } else {
                create_colored_cell(service, COLOR_ERROR).to_string()
            })
            .collect::<Vec<String>>()
            .join("\n")
    ]);

    table.printstd();
}

fn execute_system_command(command: &str, args: &[&str]) -> String {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("Failed to execute system command");

    match output.status.success() {
        true => str::from_utf8(&output.stdout)
            .unwrap_or_default()
            .trim()
            .to_string(),
        false => str::from_utf8(&output.stderr)
            .unwrap_or_default()
            .trim()
            .to_string(),
    }
}

// implementation to get the default gateway
fn get_default_gateway() -> String {
    let output = execute_system_command("ip", &["route", "show", "default"]);
    let default_route_line = output.lines().next().unwrap_or_default();
    let parts: Vec<&str> = default_route_line.split_whitespace().collect();
    parts.get(2).unwrap_or(&"").to_string()
}

// implementation to get DNS servers from /etc/resolv.conf
fn get_dns_servers() -> Vec<String> {
    let output = execute_system_command("cat", &["/etc/resolv.conf"]);
    output
        .lines()
        .filter(|line| line.contains("nameserver"))
        .map(|line| {
            line.split_whitespace()
                .nth(1)
                .unwrap_or_default()
                .to_string()
        })
        .collect()
}

fn get_vlan_interfaces() -> Vec<String> {
    let output = execute_system_command("ip", &["link", "show"]);
    let mut interfaces: Vec<String> = output
        .lines()
        .filter(|line| line.contains('@'))
        .map(|line| {
            line.split(':')
                .nth(1)
                .unwrap_or_default()
                .trim()
                .to_string()
        })
        .collect();

    // Check if the collected list is empty and insert a default message
    if interfaces.is_empty() {
        interfaces.push("No VLAN interfaces found".to_string());
    }

    interfaces
}

fn get_arp_entries() -> Result<Vec<ArpEntry>, &'static str> {
    let output = execute_system_command("arp", &["-n"]);
    let entries: Vec<_> = output
        .lines()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                Some(ArpEntry {
                    ip: parts[0].to_string(),
                    hw_type: parts[1].to_string(),
                    flags: parts[2].to_string(),
                    hw_address: parts[3].to_string(),
                    mask: parts[4].to_string(),
                    device: parts.get(5).unwrap_or(&"").to_string(),
                })
            } else {
                None
            }
        })
        .collect();

    if entries.is_empty() {
        Err("No ARP entries found.")
    } else {
        Ok(entries)
    }
}

// implementation to get interface configurations
fn get_interface_configs() -> Vec<String> {
    let output = execute_system_command("ip", &["addr"]);
    output.lines().map(|s| s.to_string()).collect()
}

// implementation to get routing table
fn get_routing_table() -> Vec<String> {
    let output = execute_system_command("ip", &["route"]);
    output.lines().map(|s| s.to_string()).collect()
}

// implementation to get listening ports
fn get_listening_ports() -> Vec<String> {
    let output = execute_system_command("ss", &["-tuln"]);
    output.lines().skip(1).map(|s| s.to_string()).collect() // Skip the header
}

// implementation to get LLDP neighbors
fn get_lldp_neighbors() -> String {
    // This is dependent on whether the lldpcli tool is installed
    // Here, we'll check if the command exists first
    let check_command = Command::new("sh")
        .arg("-c")
        .arg("command -v lldpcli")
        .output();

    match check_command {
        Ok(Output { status, .. }) if status.success() => {
            execute_system_command("lldpcli", &["show", "neighbors"])
        }
        _ => "LLDP information not available (lldpcli command not found).".to_string(),
    }
}

// implementation to perform traceroute to first DNS server
fn perform_traceroute_to_first_dns(dns_servers: &[String]) -> String {
    if let Some(dns_server) = dns_servers.first() {
        let output = execute_system_command("traceroute", &["-m", "5", dns_server]);
        output
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    } else {
        "No DNS servers found".to_string()
    }
}

fn get_reverse_dns_for_arp(arp_entries: &[ArpEntry]) -> Vec<(String, String, String)> {
    arp_entries
        .iter()
        .map(|entry| {
            let mut status_msg = "Success".to_string();
            let dns_result = if !entry.ip.is_empty() {
                let output = execute_system_command("dig", &["+short", "-x", &entry.ip]);
                if output.is_empty() {
                    status_msg = "Hostname did not resolve".to_string();
                    "N/A".to_string()
                } else {
                    output.trim().to_string()
                }
            } else {
                status_msg = "No IP address".to_string();
                "N/A".to_string()
            };
            (entry.ip.clone(), dns_result, status_msg)
        })
        .collect()
}

// implementation to check for common services on the default gateway
fn check_common_services(gateway: &str) -> Vec<String> {
    let ports = ["53", "80", "443"];
    ports
        .iter()
        .map(|port| {
            let output = Command::new("nc").args(["-zv", gateway, port]).output();

            match output {
                Ok(Output { status, .. }) if status.success() => {
                    format!("Port {} (TCP) is open on {}", port, gateway)
                }
                _ => format!("Port {} (TCP) is closed on {}", port, gateway),
            }
        })
        .collect()
}

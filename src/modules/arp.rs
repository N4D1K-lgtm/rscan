use super::Module;
use async_trait::async_trait;
use prettytable::{row, Table};
use std::error::Error;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub struct ARPModule;

#[async_trait]
impl Module for ARPModule {
    fn name(&self) -> String {
        "arp".to_string()
    }

    async fn run(&self) -> Result<Table, Box<dyn Error>> {
        let output = if cfg!(target_os = "windows") {
            Command::new("arp").arg("-a").output().await?
        } else {
            Command::new("ip").args(["neighbor"]).output().await?
        };

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Command execution failed: {}", error_message),
            )));
        }

        let stdout = std::str::from_utf8(&output.stdout)?;
        let arp_entries: Vec<&str> = stdout.lines().collect();

        let mut table = Table::new();
        table.set_titles(row![
            "IP Address",
            "Device",
            "MAC Address",
            "State",
            "Hostname"
        ]);

        if arp_entries.is_empty() {
            table.add_row(row!["No ARP/NDP entries found", "", "", "", ""]);
        } else {
            for entry in arp_entries {
                let parts: Vec<&str> = entry.split_whitespace().collect();
                if parts.len() >= 4 {
                    let ip = parts[0];
                    let device = parts[1];
                    let mac_index = parts.iter().position(|&r| r == "lladdr");
                    let mac = mac_index
                        .map(|index| parts.get(index + 1).unwrap_or(&""))
                        .unwrap_or(&"");
                    let state = parts.last().unwrap_or(&"");

                    // Reverse DNS lookup using dig
                    let hostname = if !ip.is_empty() {
                        reverse_dns_lookup(ip).await?
                    } else {
                        "N/A".to_string()
                    };

                    table.add_row(row![ip, device, mac, state, hostname]);
                }
            }
        }

        Ok(table)
    }
}

async fn reverse_dns_lookup(ip: &str) -> Result<String, Box<dyn Error>> {
    let mut command = Command::new("dig")
        .args(["+short", "-x", ip])
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = command
        .stdout
        .take()
        .ok_or("Failed to capture dig stdout")?;
    let mut reader = BufReader::new(stdout).lines();

    if let Some(line) = reader.next_line().await? {
        Ok(line.trim().to_string())
    } else {
        Ok("N/A".to_string())
    }
}


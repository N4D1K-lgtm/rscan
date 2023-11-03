use super::Module;
use async_trait::async_trait;
use prettytable::{row, Table};
use std::error::Error;
use std::str;
use tokio::process::Command;

pub struct WirelessAccessPointModule;

#[async_trait]
impl Module for WirelessAccessPointModule {
    fn name(&self) -> String {
        "Wireless Access Points".to_string()
    }

    async fn run(&self) -> Result<Table, Box<dyn Error>> {
        let output = if cfg!(target_os = "windows") {
            Command::new("netsh")
                .args(["wlan", "show", "interfaces"])
                .output()
                .await?
        } else {
            // Using nmcli to list Wi-Fi networks on Linux
            Command::new("nmcli")
                .args(["-t", "-f", "in-use,ssid,bssid,signal,freq", "dev", "wifi"])
                .output()
                .await?
        };
        // Check if the command was successful
        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Command execution failed: {}", error_message),
            )));
        }

        // Parse the output to extract currently connected SSID
        let stdout = str::from_utf8(&output.stdout)?;
        let current_ssid = get_current_ssid(stdout).ok_or("Could not determine current SSID")?;

        let access_points = parse_access_points(stdout, &current_ssid)?;

        let mut table = Table::new();
        table.add_row(row!["SSID", "BSSID", "Signal Strength", "Frequency"]);

        for ap in access_points {
            table.add_row(row![
                ap.ssid,
                ap.bssid,
                format!("{}%", ap.signal_strength),
                format!("{}MHz", ap.frequency),
            ]);
        }

        Ok(table)
    }
}

// Struct to hold access point information
struct AccessPoint {
    ssid: String,
    bssid: String,
    signal_strength: u8,
    frequency: u16,
}

fn parse_access_points(
    raw_output: &str,
    current_ssid: &str,
) -> Result<Vec<AccessPoint>, Box<dyn Error>> {
    let mut access_points = Vec::new();

    for line in raw_output.lines() {
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Split the line into parts and remove empty leading entries due to leading colons
        let parts: Vec<&str> = line.split(':').filter(|part| !part.is_empty()).collect();

        // We expect to have 4 parts after splitting: in-use, ssid, bssid, signal, freq
        if parts.len() == 4 {
            let ssid = parts[1].trim(); // Trim whitespace from the SSID
                                        // Only consider this line if the SSID matches the current SSID (and is not empty)
            if ssid == current_ssid && !ssid.is_empty() {
                let ap = AccessPoint {
                    ssid: ssid.to_string(),
                    bssid: parts[2].trim().to_string(),
                    signal_strength: parts[3].parse()?,
                    frequency: parts[4].parse()?,
                };
                access_points.push(ap);
            }
        }
    }

    Ok(access_points)
}
fn get_current_ssid(raw_output: &str) -> Option<String> {
    let current_ssid = raw_output.lines().find_map(|line| {
        if line.starts_with('*') {
            line.split(':').nth(1).map(|ssid| ssid.trim().to_string())
        } else {
            None
        }
    });

    // Debug print the current SSID
    println!("Current SSID: {:?}", current_ssid.clone().unwrap());

    current_ssid
}

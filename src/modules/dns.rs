use super::Module;
use crate::styles;
use async_trait::async_trait;
use prettytable::Table;
use std::error::Error;
use std::str;
use tokio::process::Command;

pub struct DNSServerModule;

#[async_trait]
impl Module for DNSServerModule {
    fn name(&self) -> String {
        "DNS Server".to_string()
    }

    async fn run(&self) -> Result<Table, Box<dyn Error>> {
        let output = if cfg!(target_os = "windows") {
            Command::new("powershell")
                .args(["Get-DnsClientServerAddress", "-AddressFamily", "IPv4"])
                .output()
                .await?
        } else {
            // Using nmcli to get DNS servers on Linux
            Command::new("nmcli")
                .args(["--terse", "--fields", "IP4.DNS", "device", "show"])
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

        // Parse the stdout to extract DNS servers
        // This is an example for Linux parsing, you'll need to adjust accordingly
        let stdout = str::from_utf8(&output.stdout)?;
        let dns_servers: Vec<&str> = stdout
            .lines()
            .filter(|line| line.starts_with("IP4.DNS"))
            .map(|line| line.split(':').nth(1).unwrap_or("").trim())
            .collect();

        let mut table = styles::create_styled_table(&self.name(), 2);

        if dns_servers.is_empty() {
            table.add_row(styles::labeled_row("DNS Servers", "No DNS servers found"));
        } else {
            for server in dns_servers {
                table.add_row(styles::labeled_row("DNS Server", server));
            }
        }

        Ok(table)
    }
}

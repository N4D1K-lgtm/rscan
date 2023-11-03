use super::Module;
use crate::styles;
use async_trait::async_trait;
use prettytable::Table;
use std::error::Error;
use std::str;
use tokio::process::Command;

pub struct GatewayModule;

#[async_trait]
impl Module for GatewayModule {
    fn name(&self) -> String {
        "Gateway".to_string()
    }

    async fn run(&self) -> Result<Table, Box<dyn Error>> {
        // Execute the command and capture the output

        let output = if cfg!(target_os = "windows") {
            Command::new("powershell")
                .args([
                    "-Command",
                    "Get-NetRoute -DestinationPrefix 0.0.0.0/0 | Select-Object -ExpandProperty NextHop",
                ])
                .output()
                .await?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("ip route | grep default | awk '{print $3}'")
                .output()
                .await?
        };

        if !output.status.success() {
            // Handle non-zero exit code (i.e., command execution failure)
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Command execution failed: {}", error_message.trim()),
            )));
        }
        let mut table = styles::create_styled_table("Default Gateway(s)", 2);
        let gateway_ip = str::from_utf8(&output.stdout)?.trim();

        if !gateway_ip.is_empty() {
            table.add_row(styles::labeled_row("IP Address", gateway_ip));
        } else {
            // Handle the case where the gateway IP is empty but the command execution succeeded
            table.add_row(styles::labeled_row("Gateway IP", "No gateway IP found"));
        }

        Ok(table)
    }
}

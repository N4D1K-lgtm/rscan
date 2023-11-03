use super::Module;
use async_trait::async_trait;
use prettytable::{row, Table};
use regex::Regex;
use std::error::Error;
use tokio::process::Command;

pub struct LLDPModule;

#[async_trait]
impl Module for LLDPModule {
    fn name(&self) -> String {
        "LLDP Neighbors".to_string()
    }

    async fn run(&self) -> Result<Table, Box<dyn Error>> {
        // This function assumes that lldpcli is installed and available in the PATH.
        let lldp_output = Command::new("lldpcli")
            .args(["show", "neighbors"])
            .output()
            .await?;

        if !lldp_output.status.success() {
            let error_message = String::from_utf8_lossy(&lldp_output.stderr);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("lldpcli command execution failed: {}", error_message),
            )));
        }

        let lldp_info = String::from_utf8(lldp_output.stdout)?;

        // Parse the LLDP information and create a table here.
        // For the sake of this example, let's assume the function `parse_lldp_info`
        // exists and returns a Vec of neighbor info tuples or structs.
        let neighbors = parse_lldp_info(&lldp_info)?;

        let mut table = Table::new();
        table.add_row(row![
            "Interface",
            "Chassis ID",
            "System Name",
            "System Description",
            "Management IP(s)",
            "Port ID",
            "Port Description",
            "Capabilities"
        ]);

        for neighbor in neighbors {
            // Concatenate all management IPs into a single string
            let mgmt_ips = neighbor.mgmt_ips.join(", ");

            table.add_row(row![
                neighbor.interface,
                neighbor.chassis_id,
                neighbor.sys_name,
                neighbor.sys_descr,
                mgmt_ips,
                neighbor.port_id,
                neighbor.port_descr,
                neighbor.capabilities
            ]);
        }

        Ok(table)
    }
}

fn parse_lldp_info(lldp_info: &str) -> Result<Vec<NeighborInfo>, Box<dyn Error>> {
    let mut neighbors = Vec::new();
    let re_section = Regex::new(
        r"Interface: +(?P<interface>.+?),(?s:.+?)Capability: +(?P<caps>.+?)\n(?s:.+?Port:)?",
    )?;
    let re_chassis = Regex::new(
        r"ChassisID: +(?P<chassisid>.+?)\n +SysName: +(?P<sysname>.+?)\n +SysDescr: +(?P<sysdescr>.+?)\n",
    )?;
    let re_mgmt_ip = Regex::new(r"MgmtIP: +(?P<mgmtip>[^\n]+)")?;
    let re_port = Regex::new(r"PortID: +(?P<portid>.+?)\n +PortDescr: +(?P<portdescr>.+?)\n")?;
    let re_capabilities = Regex::new(r"Capability: +(\w+), +(\w+)")?;

    for cap in re_section.captures_iter(lldp_info) {
        let interface = cap["interface"].trim().to_string();
        let capabilities_matches = re_capabilities.find_iter(&cap[0]); // Capturing capabilities within the interface section

        let chassis_match = re_chassis
            .captures(lldp_info)
            .ok_or("Chassis information not found")?;
        let port_match = re_port
            .captures(lldp_info)
            .ok_or("Port information not found")?;

        // In a real-world scenario, there may be multiple management IPs. Adjust the logic as necessary.
        let mgmt_ips: Vec<String> = re_mgmt_ip
            .find_iter(lldp_info)
            .map(|mat| mat.as_str().replace("MgmtIP:", "").trim().to_string())
            .collect();

        // Filter and collect only the capabilities that are 'on'
        let capabilities: Vec<String> = capabilities_matches
            .filter_map(|caps_match| {
                let caps_capture = re_capabilities.captures(caps_match.as_str())?;
                if caps_capture.get(2)?.as_str() == "on" {
                    Some(caps_capture.get(1)?.as_str().to_string())
                } else {
                    None
                }
            })
            .collect();

        let neighbor = NeighborInfo {
            interface,
            chassis_id: chassis_match["chassisid"].trim().to_string(),
            sys_name: chassis_match["sysname"].trim().to_string(),
            sys_descr: chassis_match["sysdescr"].trim().to_string(),
            mgmt_ips,
            port_id: port_match["portid"].trim().to_string(),
            port_descr: port_match["portdescr"].trim().to_string(),
            capabilities: capabilities.join(", "),
        };

        neighbors.push(neighbor);
    }

    Ok(neighbors)
}

// Struct to hold neighbor information - adjust fields as necessary
struct NeighborInfo {
    interface: String,
    chassis_id: String,
    sys_name: String,
    sys_descr: String,
    mgmt_ips: Vec<String>,
    port_id: String,
    port_descr: String,
    capabilities: String,
}

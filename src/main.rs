use rscan_core::prelude::*;
use rscan_derive::module;

pub fn main() {
    rscan_ui::setup();
}

#[module(
    "Configuration Backup",
    "Backs up configurations of network devices.",
    "Lebron James",
    "1.0",
    "Utility Tools"
)]
fn configuration_backup_module() -> ModuleResult {
    // Implementation
    let mut table = prettytable::Table::new();
    table.add_row(prettytable::row!["Device", "Backup Status"]);
    // Example data
    table.add_row(prettytable::row!["Switch", "Success"]);
    Ok(table)
}

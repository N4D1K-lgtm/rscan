use async_trait::async_trait;
use futures::stream::{FuturesOrdered, StreamExt};
use prettytable::Table;
use std::error::Error;

mod arp;
mod dns;
mod gateway;
mod lldp;
mod wifi;

use arp::ARPModule;
use dns::DNSServerModule;
use gateway::GatewayModule;
use lldp::LLDPModule;
use wifi::WirelessAccessPointModule;

#[async_trait]
pub trait Module {
    fn name(&self) -> String;
    async fn run(&self) -> Result<Table, Box<dyn Error>>;
}

// ModuleManager struct that holds all modules
pub struct ModuleManager {
    modules: Vec<Box<dyn Module>>,
}

impl ModuleManager {
    pub fn new() -> Self {
        ModuleManager {
            modules: vec![
                Box::new(GatewayModule),
                Box::new(DNSServerModule),
                Box::new(ARPModule),
                Box::new(LLDPModule),
            ],
        }
    }

    // Get names of all modules
    pub fn get_module_names(&self) -> Vec<String> {
        self.modules.iter().map(|m| m.name()).collect()
    }

    pub async fn run_selected_modules(
        &self,
        selected: &[String],
    ) -> Result<Vec<prettytable::Table>, Box<dyn Error>> {
        let mut futures = FuturesOrdered::new();

        for module in &self.modules {
            if selected.contains(&module.name()) {
                futures.push_back(module.run());
            }
        }

        let mut tables = Vec::new();
        while let Some(result) = futures.next().await {
            match result {
                Ok(table) => tables.push(table),
                Err(e) => return Err(e),
            }
        }

        Ok(tables)
    }
    // Add a method to print all tables
    pub fn print_tables(tables: Vec<prettytable::Table>) {
        for table in tables {
            table.printstd();
            println!();
        }
    }
}

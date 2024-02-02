use prettytable::Table;
use thiserror::Error;

mod utils;
pub use utils::*;

pub mod prelude;

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("Module failed to execute")]
    ExecutionError,
}

pub type ModuleResult = Result<Table, ModuleError>;

pub enum ModuleKind {
    Sync(fn() -> ModuleResult),
}

pub enum Platform {
    Windows,
    Linux,
    MacOS,
    All,
}

pub fn parse_platforms(platforms: &str) -> Vec<Platform> {
    let mut result = Vec::new();
    for platform in platforms.split(',') {
        match platform.trim().to_lowercase().as_str() {
            "windows" => result.push(Platform::Windows),
            "linux" => result.push(Platform::Linux),
            "macos" => result.push(Platform::MacOS),
            "all" => result.push(Platform::All),
            _ => {
                panic!("Invalid platform: {}", platform);
            }
        }
    }
    result
}

pub struct Module {
    // Display name of the module ex. "Default Gateway"
    pub name: &'static str,
    // unique identifier for the module ex. "default_gateway"
    pub identifier: &'static str,
    pub description: &'static str,
    pub author: &'static str,
    pub version: &'static str,
    // category of the module ex. "Active Directory", "Passive Discovery"
    pub category: &'static str,
    // async or sync
    pub kind: ModuleKind,
    // platform support
    pub platforms: Vec<Platform>,
}

impl Module {
    pub async fn execute(&self) -> ModuleResult {
        match &self.kind {
            ModuleKind::Sync(func) => func(),
        }
    }
}

inventory::collect!(Module);

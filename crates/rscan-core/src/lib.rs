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

pub type ModuleResult = Box<Result<Table, ModuleError>>;

pub enum ModuleKind {
    Sync(fn() -> ModuleResult),
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
}

impl Module {
    pub async fn execute(&self) -> ModuleResult {
        match &self.kind {
            ModuleKind::Sync(func) => func(),
        }
    }
}

inventory::collect!(Module);

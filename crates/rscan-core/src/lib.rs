use async_trait::async_trait;
use prettytable::{row, Table};
use std::fmt::{Display, Formatter};
use thiserror::Error;

mod utils;

// pub use utils::*;

pub mod prelude;

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("Module failed to execute")]
    ExecutionError,
}

pub type ModuleResult = Result<Table, ModuleError>;

pub trait Module {
    fn name(&self) -> &'static str;
    fn identifier(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn author(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn category(&self) -> &'static str;
}

impl Display for dyn Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}",
            self.name(),
            self.identifier(),
            self.description(),
            self.author(),
            self.version(),
            self.category()
        )
    }
}

impl Display for dyn GlobalAsyncModule + Send + Sync {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}",
            self.name(),
            self.identifier(),
            self.description(),
            self.author(),
            self.version(),
            self.category()
        )
    }
}

#[async_trait]
pub trait GlobalAsyncModule: Module + Send + Sync {
    async fn run_async(&self) -> ModuleResult;
}

#[async_trait]
pub trait LocalAsyncModule: Module {
    async fn run_async(&self) -> ModuleResult;
}

pub trait BlockingModule: Module {
    fn run_blocking(&self) -> ModuleResult;
}
/// Enum to represent the different execution types of modules.
pub enum ModuleExecution {
    Local(Box<dyn LocalAsyncModule>),
    Global(Box<dyn GlobalAsyncModule + Send + Sync>),
    Blocking(Box<dyn BlockingModule>),
}

pub struct ModuleHolder {
    pub identifier: &'static str,
    pub constructor: fn() -> ModuleExecution,
}

inventory::collect!(ModuleHolder);

#[macro_export]
macro_rules! define_module {
    // Define an AsyncGlobal module (default if not specified).
    (
        identifier: $identifier:expr,
        category: $category:expr,
        async fn $fn_name:ident($($arg_name:ident: $arg_ty:ty),*) -> $ret_ty:ty $body:block
    ) => {
        struct $fn_name;

        impl Module for $fn_name {
            fn name(&self) -> &'static str {
                stringify!($fn_name)
            }
            fn identifier(&self) -> &'static str {
                $identifier
            }
            fn description(&self) -> &'static str {
                "Automatically generated description"
            }
            fn author(&self) -> &'static str {
                "Auto Author"
            }
            fn version(&self) -> &'static str {
                "1.0"
            }
            fn category(&self) -> &'static str {
                $category
            }
        }

        #[async_trait::async_trait]
        impl GlobalAsyncModule for $fn_name {
            async fn run_async(&self) -> ModuleResult {
                $body
            }
        }

        fn constructor() -> ModuleExecution {
            ModuleExecution::Global(Box::new($fn_name {}))
        }

        inventory::submit! {
            ModuleHolder {
                identifier: $identifier,
                constructor: constructor,
            }
        }
    }; // Definitions for AsyncLocal and Blocking could follow a similar pattern.
}

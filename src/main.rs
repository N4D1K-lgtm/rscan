use prettytable::{row, Table};
use rscan_core::define_module;
use rscan_core::prelude::*;

#[tokio::main]
pub async fn main() {
    rscan_ui::setup();

    inventory::iter::<ModuleHolder>.into_iter().for_each(|m| {
        let module = (m.constructor)();
        match module {
            ModuleExecution::Local(module) => {
                println!("Local module: {}", module.name());
            }
            ModuleExecution::Global(module) => {
                tokio::spawn(async move {
                    println!("Global module: {}", module);
                    println!("{}", module.run_async().await.unwrap())
                });
            }
            ModuleExecution::Blocking(module) => {
                println!("Blocking module: {}", module.name());
            }
        }
    });
}

define_module! {
    identifier: "example_module",
    category: "Example",
    async fn ExampleModule() -> ModuleResult {
        let mut table = Table::new();
        table.add_row(row!["Name", "Age"]);
        table.add_row(row!["John", "20"]);
        Ok(table)
    }
}

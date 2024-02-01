use clap::Parser;

use rscan_core::prelude::*;
use rscan_derive::module;

mod cli;

use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    println!("{:?}", get_names().await);

    for result in execute_all().await {
        match result {
            Ok(table) => println!("{}", table),
            Err(e) => println!("Error running module {}", e),
        }
    }
}
#[module("Dummy Module")]
fn example_module() -> ModuleResult {
    let mut table = prettytable::Table::new();
    table.add_row(prettytable::row!["Hello", "World"]);
    Ok(table)
}

#[module]
fn nate_is_silly() -> ModuleResult {
    let mut table = prettytable::Table::new();
    table.add_row(prettytable::row!["nate", "was", "here"]);
    Ok(table)
}

#[module("New Module")]
fn new_module() -> ModuleResult {
    let mut table = prettytable::Table::new();
    table.add_row(prettytable::row!["New", "Module"]);
    Ok(table)
}

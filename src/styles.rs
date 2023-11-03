use prettytable::{format::Alignment, *};

const COLOR_SUCCESS: color::Color = color::GREEN;
const COLOR_WARNING: color::Color = color::YELLOW;
const COLOR_ERROR: color::Color = color::RED;
const COLOR_INFO: color::Color = color::BRIGHT_GREEN;
const COLOR_HIGHLIGHT: color::Color = color::BLUE;

// style.rs
use prettytable::{cell, row, Cell, Row, Table};

pub fn header_cell(content: &str) -> Cell {
    cell!(b -> content)
}

pub fn normal_cell(content: &str) -> Cell {
    cell!(content)
}

pub fn error_cell(content: &str) -> Cell {
    cell!(Fr -> content)
}

pub fn create_module_separator(title: &str) -> Row {
    Row::new(vec![header_cell(title)])
}

pub fn labeled_row(label: &str, content: &str) -> Row {
    row![b -> label, content]
}

// You might also want to create a function to construct a whole table for a module if necessary
pub fn create_module_table() -> Table {
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table
}

// Functions to add rows to a table, etc.
pub fn add_row_to_table(table: &mut Table, row: Row) {
    table.add_row(row);
}

pub fn create_styled_table(module_name: &str, span: usize) -> Table {
    let title_cell = Cell::new_align(module_name, Alignment::CENTER).with_hspan(span);
    let title_row = Row::new(vec![title_cell]);

    let mut table = Table::new();
    table.set_titles(title_row);

    table
}

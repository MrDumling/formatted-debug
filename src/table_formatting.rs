//! # Table Formatting
//!
//! `table_formatting` is a module to format values 
//! into vectors of strings which represent tables

pub mod grid_formatting;
pub mod string_grid;

pub trait StringTable {
    fn to_table(&self) -> Vec<String>;
}
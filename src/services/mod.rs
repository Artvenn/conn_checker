mod tests;

use crate::models::{*, config::Strl, address::Address};

pub fn read_file(path: &str) -> Result<String, Strl> {unimplemented!()}
pub fn map_to_addresses(file_text: String) -> Result<Vec<Address>, Strl> {unimplemented!();}
pub fn filter_working(addresses: Vec<Address>) -> Result<Vec<Address>, Strl> {unimplemented!();}
pub fn print_working(addresses: &Vec<Address>) {unimplemented!()}
pub fn save_working(addresses: Vec<Address>) -> Result<(), Strl> {unimplemented!();}
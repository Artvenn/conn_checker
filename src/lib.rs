#![allow(unused)]

pub mod services;
pub mod models;
pub mod constants;

use std::process;

use models::config::Config;
use crate::services::*;

pub fn run(config: Config) {
    let file_text =  read_file(&config.file_path).unwrap_or_else(|err| {
        println!("{err}"); 
        process::exit(1);
    });
    let working = get_working(&file_text, &config);
    if working.is_empty(){
        println!("There is no working");
        process::exit(1);
    }
    print_working(&working);
    if let Err(err) = save_working(working) {
        println!("Cant save result to file. {err}");
    }
}
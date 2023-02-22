#![allow(unused)]

pub mod services;
pub mod models;
pub mod constants;

use models::config::Config;

pub fn run(config: Config) {
    println!("{:#?}", config);

    // let file_text = read_file(&config.file_path).unwrap();
    // let addresses =  map_to_addresses(file_text).unwrap();
    // let working = filter_working(addresses).unwrap();
    // print_working(&working);
    // save_working(working).unwrap();
}
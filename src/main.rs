#![allow(unused)]

use std::env;
use std::process;
use conn_checker::{constants::{hints}, models::Config, run};

fn main() {
    let mut args_iter = env::args();
    args_iter.next();

    if let Some(val) = args_iter.next() {
        if val == "--help" {
            println!("{}", hints::HELP_HINT);
            return;
        }
    };

    let config = Config::build(env::args()).unwrap_or_else(|err_msg| {
        println!("{err_msg}");
        println!("use --help for getting available flags");
        process::exit(1);
    });
    run(config);
}
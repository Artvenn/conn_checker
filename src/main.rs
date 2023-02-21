#![allow(unused, dead_code, non_snake_case)]

use std::{env, process, ops::Add, iter::Map};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err_msg| {
        println!("{err_msg}");
        process::exit(1);
    });
    run(config);
}


fn run(config: Config) {
    println!("Got config {:#?}", config)
}
fn read_file(file_path: &String) -> Result<String, &'static str> {unimplemented!()}
fn parse_file<'a>( file_text: &'a str) -> impl Iterator<Item = Address<'a>>  {
    file_text.split(" ").map(|el| {Address { ip: "231231", port: 80 }})
}
fn check_connection(addr: &Address) -> bool {unimplemented!();}
fn print_working<'a>(addresses: &Vec<Address<'a>>) {unimplemented!()}
fn save_working<'a>(addresses: &Vec<Address<'a>>) -> Result<(), &'static str> {unimplemented!()}

struct Address<'a> {
    ip: &'a str,
    port: i32
}


#[derive(PartialEq, Debug)]
struct Config {
    file_path: String,
    default_port: Option<i32>
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) 
    -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(path) => path,
            None => return Err(ErrMsgs::MISS_PATH_MSG)
        };
        let default_port = match args.next() {
            Some(port) => Some(
                match port.parse::<i32>() {
                    Ok(res) => res,
                    Err(err) => return Err(ErrMsgs::INVALID_PORT_MSG)
                },
            ),
            None => None
        };

        Ok(Config{file_path, default_port})
    }
}

mod ErrMsgs {
    pub const MISS_PATH_MSG: &str = "Argument error: missing path to address file";
    pub const INVALID_PORT_MSG: &str = "Argument error: port must be a positive integer in range 1..65535";
}

#[cfg(test)]
mod config__build {
    use super::*;
    #[test]
    fn should_build_struct_1() {
        let file_path = "./some/path".to_string();
        let default_port = "8080".to_string();
        let args_fixture = [
            "prog_name".to_string(), 
            file_path.clone(), 
            default_port.clone()
        ].into_iter();

        let expect: Result<Config, &'static str> = Ok(Config {
            file_path,
            default_port: Some(default_port.parse::<i32>().unwrap())
        });
        let actual = Config::build(args_fixture);
        assert_eq!(actual, expect);
    }
    #[test]
    fn should_build_struct_2() {
        let file_path = "path/some/path".to_string();
        let args_fixture = [
            "prog_name".to_string(), 
            file_path.clone(), 
        ].into_iter();

        let expect: Result<Config, &'static str> = Ok(Config {
            file_path,
            default_port: None
        });
        let actual = Config::build(args_fixture);
        assert_eq!(actual, expect);
    }
    #[test]
    fn should_return_err_mesg_1() {
        let args_fixture = [
            "prog_name".to_string(), 
        ].into_iter();

        let expect = Err("Argument error: missing path to address file");
        let actual = Config::build(args_fixture);
        assert_eq!(actual, expect);
    }
    #[test]
    fn should_return_err_msg_2() {
        let file_path = "./some/path".to_string();
        let default_port = "a808d0".to_string();
        let args_fixture = [
            "prog_name".to_string(), 
            file_path.clone(), 
            default_port.clone()
        ].into_iter();

        let expect = Err("Argument error: port must be a positive integer in range 1..65535");
        let actual = Config::build(args_fixture);
        assert_eq!(actual, expect);
    }
}
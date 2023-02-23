use std::{fs, io::{ErrorKind, self}, net::{TcpStream, SocketAddr}, time::Duration, str::FromStr, error::Error};

use crate::models::{*, config::Strl};

pub fn read_file(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(data) => Ok(data),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => return Err(format!("File: {path}, is not found")),
            err => return Err(format!("Error reading file {path} >> {err}"))
        }
    }
}

pub fn get_working<'a, 'b>(file_text: &'a String, config: &'b Config) -> Vec<&'a str> {
    let filtered: Vec<&str> = file_text.lines().filter(|line| {
        let sock_addr = match config.default_port {
            Some(port) => {
                SocketAddr::from_str(
                    format!("{}:{}", line.split(":").collect::<Vec<&str>>().first().unwrap(), 
                        port.to_string()).as_str())
            },
            None =>  {
                SocketAddr::from_str(line)
            }
        };

        if let Some(_) = config.ignore_errs {
            if let Err(_) = sock_addr {return false}
        } 

        if let Err(err) = sock_addr {
            panic!("Address parsing error: {line}. {err}")
        };
        match TcpStream::connect_timeout(&sock_addr.unwrap(), Duration::from_secs(3)) {
            Ok(_) => {
                if let Some(()) =  config.verbose {
                    println!("[Ok]\t{line}");   
                } 
                true
            },
            Err(err) => {
                if let Some(()) = config.verbose {
                    println!("[Err]\t{line}");
                }
                false
            }
        }
    }).collect();
    filtered
}

pub fn print_working(addresses: &Vec<&str>) {
    println!("---------------------------Connected list----------------------------------");
    for addr in addresses {
        println!("{addr}")
    }
    println!("---------------------------------------------------------------------------");
}

pub fn save_working(addresses: Vec<&str>) -> io::Result<()> {
    fs::write("./working.txt", addresses.join("\n"))
}
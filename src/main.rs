use std::env;
use std::process;

use conn_checker::constants::{hints};
use conn_checker::models::Config;
use conn_checker::run;
// 1. Parse command line arguments.
// 2. Read file by specified path.
// 3. Parse file to ip:port
// 4. Check connection with each pair of ip:port
// 5. Print working if there have
// 6. Save working to file, if have there.
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
        process::exit(1);
    });
    run(config);
}
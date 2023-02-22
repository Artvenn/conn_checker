use crate::constants::err_msgs;

pub type Strl = &'static str;

#[derive(PartialEq, Debug)]
pub struct Config {
    pub file_path: String,
    pub ignore_errs: bool
}

impl Config {
    pub fn build<T>(mut args: T) -> Result<Self, Strl>
        where T : Iterator<Item = String> 
    {
        args.next();
        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => return Err(err_msgs::MISS_REQ_ARG)
        };
        let ignore_errs = match args.next() {
            Some(flag) => {
                if flag == "--ignore_errs" {
                    true
                } else {
                    return Err(err_msgs::INVALID_FLAG)
                }
            }
            None => false
        };
        Ok(Config {file_path, ignore_errs})
    }
}
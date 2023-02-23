use crate::constants::{err_msgs, flags};

pub type Strl = &'static str;

#[derive(PartialEq, Debug)]
pub struct Config {
    pub file_path: String,
    pub default_port: Option<u16>,
    pub ignore_errs: Option<()>,
    pub verbose: Option<()>
}

impl Config {
    pub fn build<T>(arg_tokens: T) -> Result<Self, String>
        where T : Iterator<Item = String> 
    {
        let arg_tokens: Vec<String> = arg_tokens.collect();
        let mut file_path_i: Option<usize> = None;
        let mut default_port_i: Option<usize> = None;
        let mut ignore_errs_i: Option<usize> = None;
        let mut verbose_i: Option<usize> = None;
        let mut conf = Self {
            file_path: "".to_string(),
            default_port: None,
            ignore_errs: None,
            verbose: None,
        };

        for (i, token) in arg_tokens.iter().enumerate() {
            if token == flags::PATH {
                file_path_i = Some(i);
            } else if token == flags::DEFAULT_PORT {
                default_port_i = Some(i);
            } else if token == flags::IGNORE_ERRS {
                ignore_errs_i =  Some(i);
            } else if token == flags::VERBOSE {
                verbose_i = Some(i);
            }
        }

        if let Some(ind) = file_path_i {
            if ind == arg_tokens.len() - 1 {
                return Err(make_mis_val_err(flags::PATH));
            }
            if arg_tokens[ind+1].starts_with("-") {
                return Err(make_invalid_val_err(flags::PATH));
            }
            conf.file_path = arg_tokens[ind+1].clone();
        } else {
            return Err(err_msgs::MISS_REQ_ARG.to_string());
        }

        if let Some(ind) = default_port_i {
            if ind == (arg_tokens.len() - 1) {
                return Err(make_mis_val_err(flags::DEFAULT_PORT));
            }
            if arg_tokens[ind+1].starts_with("-") {
                return Err(make_invalid_val_err(flags::DEFAULT_PORT));
            }
            conf.default_port = match arg_tokens[ind+1].parse() {
                Ok(port) => Some(port),
                Err(_) => {
                    return Err(err_msgs::PORT_REQS.to_string())
                }
            }
        } 

        if let Some(ind) = ignore_errs_i {
            conf.ignore_errs = Some(());
        } 
        if let Some(ind) = verbose_i {
            conf.verbose = Some(());
        } 

        return Ok(conf);
        fn make_mis_val_err(value: &str) -> String {
            format!("Missing value for {} flag", value)
        }
        fn make_invalid_val_err(value: &str) -> String {
            format!("Invalid value for {} flag", value)
        }
    }
}
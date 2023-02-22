type Strl = &'static str;

pub mod hints {
    use super::Strl;
    pub const HELP_HINT: Strl = "<./file_path> [default_port, --ignore_errs]";
}

pub mod err_msgs{
    use super::Strl;
    pub const MISS_REQ_ARG: Strl = "Argument error. Missing required argument: <./file_path>\n use --help to see available parameters";
    pub const INVALID_FLAG: Strl = "Invalid flag";
    pub const PORT_REQS: Strl = "Port must be a positive number in range 1..65535";
}
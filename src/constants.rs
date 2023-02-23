type Strl = &'static str;

pub mod hints {
    use super::Strl;
    pub const HELP_HINT: Strl = "--file_path [--def_port, --ignore_errs, -v]";
}

pub mod err_msgs{
    use super::Strl;
    pub const MISS_REQ_ARG: Strl = "Argument error. Missing required param: --file_path\n\
        use --help to see available parameters";
    pub const INVALID_FLAG: Strl = "Invalid flag";
    pub const PORT_REQS: Strl = "Port must be a positive number in range 1..65535";
}

pub mod flags {
    use super::Strl;
    pub const PATH: Strl = "--file_path";
    pub const DEFAULT_PORT: Strl = "--def_port";
    pub const IGNORE_ERRS: Strl = "--ignore_errs";
    pub const VERBOSE: Strl = "-v";
}
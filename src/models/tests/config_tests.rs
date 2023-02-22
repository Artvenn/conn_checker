#[cfg(test)]
mod config_m_build {
    use crate::{constants::{self, err_msgs}, models::config::Config};

    use super::*;
    //////////// Success cases ////////////////////
    #[test]
    fn should_build_with_one_file_path_arg_1() {
        ////// Arrange /////
        let file_path = String::from("./path");
        let fixture_iter = vec![
            "prog_name".to_string(),
            file_path.clone(),
        ].into_iter(); 
        let expect = Ok(Config {
            file_path,
            ignore_errs: false, 
        });
        ///////// Act ///////////
        let actual = Config::build(fixture_iter);
        //////// Assert ////////
        assert_eq!(actual, expect);
    }
    #[test]
    fn should_build_with_one_file_path_arg_2() {
        ////// Arrange /////
        let file_path = String::from("new_path/sdf");
        let fixture_iter = vec![
            "prog_name".to_string(),
            file_path.clone(),
        ].into_iter(); 
        let expect = Ok(Config {
            file_path,
            ignore_errs: false, 
        });
        ///////// Act ///////////
        let actual = Config::build(fixture_iter);
        //////// Assert ////////
        assert_eq!(actual, expect);
    }
    #[test]
    fn should_build_with_two_args() {
        ////// Arrange /////
        let file_path = String::from("./path_to_folder");
        let ignore_errs = String::from("--ignore_errs");
        let fixture_iter = vec![
            "prog_name".to_string(),
            file_path.clone(),
            ignore_errs.clone()
        ].into_iter(); 
        let expect = Ok(Config {
            file_path,
            ignore_errs: true, 
        });
        ///////// Act ///////////
        let actual = Config::build(fixture_iter);
        //////// Assert ////////
        assert_eq!(actual, expect);
    }
    /////////// Error cases //////////////////////
    #[test]
    fn should_return_missing_args_error() {
        ////// Arrange /////
        let fixture_iter = vec![
            "prog_name".to_string(),
        ].into_iter(); 
        let expect = Err(err_msgs::MISS_REQ_ARG);
        ///////// Act ///////////
        let actual = Config::build(fixture_iter);
        //////// Assert ////////
        assert_eq!(actual, expect);
    }
    #[test]
    fn should_return_invalid_flag_error() {
        ////// Arrange /////
        let file_path = String::from("./path_to_folder");
        let invalid_ign_err_flag = String::from("-nore_errs");
        let fixture_iter = vec![
            "prog_name".to_string(),
            file_path.clone(),
            invalid_ign_err_flag 
        ].into_iter(); 
        let expect = Err(err_msgs::INVALID_FLAG);
        ///////// Act ///////////
        let actual = Config::build(fixture_iter);
        //////// Assert ////////
        assert_eq!(actual, expect);
    }
}
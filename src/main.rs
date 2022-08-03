use std::{env, process};
mod parse_and_printout;
mod resolve_input;
use parse_and_printout::ParseRes;
use resolve_input::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let config_res = resolve_input::run_ztcm(config).unwrap_or_else(|err| {
        println!("Problem parsing file paths: {}", err);
        process::exit(1);
    });

    ParseRes::parse_and_print_out(config_res.0, config_res.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_args() {
        let args = Config::new_args(&["ztcm".to_string(), "./test".to_string()]).unwrap();
        assert_eq!(args.query, "./test")
    }

    #[test]
    fn new_r_args() {
        let args = Config::new_args(&["ztcm".to_string(), "./test".to_string(), "-r".to_string()])
            .unwrap();
        assert_eq!(args.query, "./test");
        assert_eq!(args.flags, vec!["-r".to_string()])
    }

    #[test]
    fn new_w_args() {
        let args = Config::new_args(&[
            "ztcm".to_string(),
            "./test".to_string(),
            "-w".to_string(),
            "2".to_string(),
        ])
        .unwrap();
        assert_eq!(args.query, "./test");
        assert_eq!(args.flags, vec!["-w".to_string(), "2".to_string()])
    }

    #[test]
    fn run_z() {
        let mut test_output_found = (false, false);
        let test_output_expected = ("test/test.module.css", 0.0);
        let config: Config = Config::new_args(&["ztcm".to_string(), "test".to_string()]).unwrap();
        let run_res = resolve_input::run_ztcm(config).unwrap();
        for pathname in run_res.0 {
            if pathname == test_output_expected.0 {
                test_output_found.0 = true;
            }
        }
        if run_res.1 == test_output_expected.1 {
            test_output_found.1 = true;
        }
        assert_eq!(test_output_found, (true, true))
    }

    #[test]
    fn run_r_z() {
        let mut test_output_found = (false, false, false);
        let test_output_expected = (
            "test/test.module.css",
            "test/recursive_test/test_r.module.css",
            0.0,
        );
        let config: Config =
            Config::new_args(&["ztcm".to_string(), "test".to_string(), "-r".to_string()]).unwrap();
        let run_res = resolve_input::run_ztcm(config).unwrap();
        for pathname in run_res.0 {
            if pathname == test_output_expected.0 {
                test_output_found.0 = true;
            } else if pathname == test_output_expected.1 {
                test_output_found.1 = true;
            }
        }
        if run_res.1 == test_output_expected.2 {
            test_output_found.2 = true;
        }
        assert_eq!(test_output_found, (true, true, true))
    }

    #[test]
    fn run_w_z() {
        let mut test_output_found = (false, false);
        let test_output_expected = ("test/test.module.css", 1.0);
        let config: Config =
            Config::new_args(&["ztcm".to_string(), "test".to_string(), "-w".to_string()]).unwrap();
        let run_res = resolve_input::run_ztcm(config).unwrap();
        for pathname in run_res.0 {
            if pathname == test_output_expected.0 {
                test_output_found.0 = true;
            }
            if run_res.1 == test_output_expected.1 {
                test_output_found.1 = true;
            }
            assert_eq!(test_output_found, (true, true))
        }
    }
}

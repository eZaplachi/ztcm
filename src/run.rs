use std::{process, time, thread, io};
// Brings flush in scope for load message
use std::io::Write; 
mod parse_and_printout;
mod resolve_input;
use parse_and_printout::parse_and_print;
use resolve_input::Config;

pub fn run_ztcm(args: &[String]) {
    let config = Config::new_args(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let data_res = Config::get_files(config).unwrap_or_else(|err| {
        println!("Problem parsing file paths: {}", err);
        process::exit(1);
    });

    parse_and_print_out(data_res.0, data_res.1, data_res.2);
}

fn parse_and_print_out(path_names: Vec<String>, camel_case_flag: bool, watch_delay: f64) {
    for path in path_names.clone() {
        println!("\nFound: {}", path);
    }
    if watch_delay != 0.0 {
        let delay = time::Duration::from_secs_f64(watch_delay);
        let mut load_state = 0;
        let mut _load_char = "";
        let mut i = 0;
        loop {
            parse_and_print(&path_names, camel_case_flag);
            thread::sleep(delay);
            // Loading icon logic
            if i > 3 {
                i = 0;
                if load_state == 3 {
                    load_state = 0;
                }
                _load_char = match load_state {
                    0 => "/",
                    1 => "-",
                    2 => "\\",
                    3 => "|",
                    _ => "*",
                };
                load_state += 1;
                print!("\r[{}]", _load_char);
                io::stdout().flush().expect("Could not flush stdout");
            }
            i += 1;
        }
    } else {
        parse_and_print(&path_names, camel_case_flag);
    }
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
    fn data_z() {
        let mut test_output_found = (false, false);
        let test_output_expected = ("test/test.module.css", 0.0);
        let config: Config = Config::new_args(&["ztcm".to_string(), "test".to_string()]).unwrap();
        let data_res = Config::get_files(config).unwrap();
        for pathname in data_res.0 {
            if pathname == test_output_expected.0 {
                test_output_found.0 = true;
            }
        }
        if data_res.2 == test_output_expected.1 {
            test_output_found.1 = true;
        }
        assert_eq!(test_output_found, (true, true))
    }

    #[test]
    fn data_r_z() {
        let mut test_output_found = (false, false, false);
        let test_output_expected = (
            "test/test.module.css",
            "test/recursive_test/test_r.module.css",
            0.0,
        );
        let config: Config =
            Config::new_args(&["ztcm".to_string(), "test".to_string(), "-r".to_string()]).unwrap();
        let data_res = Config::get_files(config).unwrap();
        for pathname in data_res.0 {
            if pathname == test_output_expected.0 {
                test_output_found.0 = true;
            } else if pathname == test_output_expected.1 {
                test_output_found.1 = true;
            }
        }
        if data_res.2 == test_output_expected.2 {
            test_output_found.2 = true;
        }
        assert_eq!(test_output_found, (true, true, true))
    }

    #[test]
    fn data_w_z() {
        let mut test_output_found = (false, false);
        let test_output_expected = ("test/test.module.css", 1.0);
        let config: Config =
            Config::new_args(&["ztcm".to_string(), "test".to_string(), "-w".to_string()]).unwrap();
        let data_res = Config::get_files(config).unwrap();
        for pathname in data_res.0 {
            if pathname == test_output_expected.0 {
                test_output_found.0 = true;
            }
            if data_res.2 == test_output_expected.1 {
                test_output_found.1 = true;
            }
            assert_eq!(test_output_found, (true, true))
        }
    }
}

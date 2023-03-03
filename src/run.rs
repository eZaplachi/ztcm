use std::{io, process, thread, time};
// Brings stdout flush in scope for load message
use std::io::Write;
mod parse_and_printout;
mod resolve_input;
use parse_and_printout::{parse_and_print, ModFlags};
use resolve_input::{Config, RunData};

pub fn run_ztcm(args: &[String]) {
    let config = Config::new_args(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let data_res = RunData::find_files(config).unwrap_or_else(|err| {
        println!("Problem parsing file paths: {}", err);
        process::exit(1);
    });

    parse_and_print_out(data_res, args);
}

fn parse_and_print_out(data: RunData, initial_args: &[String]) {
    for path in data.paths.clone() {
        println!("\nFound: {}", path);
    }
    if data.watch_delay != 0.0 {
        let delay = time::Duration::from_secs_f64(data.watch_delay);
        let mut load_state = 0;
        let mut _load_char = "";
        let mut i = 0;
        loop {
            parse_and_print(
                &data.paths,
                ModFlags {
                    camel_case_flag: data.cc_flag,
                    kebab_case_flag: data.kc_flag,
                    out_dir: &data.out_dir,
                },
            );
            thread::sleep(delay);
            // Loading icon logic
            if i % 2 == 0 {
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
            if i > data.cycles_per_refresh {
                i = 0;
                print!("\n\nRe-Indexing Files");
                run_ztcm(initial_args);
            }
            i += 1;
        }
    } else {
        parse_and_print(
            &data.paths,
            ModFlags {
                camel_case_flag: data.cc_flag,
                kebab_case_flag: data.kc_flag,
                out_dir: &data.out_dir,
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn new_args() {
        let binding = ["ztcm".to_string(), "./test".to_string()];
        let args = Config::new_args(&binding).unwrap();
        assert_eq!(args.query, "./test")
    }

    #[test]
    fn new_rec_args() {
        let binding = ["ztcm".to_string(), "./test".to_string(), "-r".to_string()];
        let args = Config::new_args(&binding).unwrap();
        assert_eq!(args.query, "./test");
        assert_eq!(args.flags, vec![&"-r".to_string()])
    }

    #[test]
    fn new_watch_args() {
        let binding = [
            "ztcm".to_string(),
            "./test".to_string(),
            "-w".to_string(),
            "2".to_string(),
        ];
        let args = Config::new_args(&binding).unwrap();
        assert_eq!(args.query, "./test");
        assert_eq!(args.flags, vec![&"-w".to_string(), &"2".to_string()])
    }

    #[test]
    fn data() {
        let mut test_output_found = (false, false);
        let test_output_expected = ("test/test.module.css", 0.0);
        let binding = ["ztcm".to_string(), "test".to_string()];
        let config: Config = Config::new_args(&binding).unwrap();
        let data_res = RunData::find_files(config).unwrap();
        for pathname in data_res.paths {
            if pathname == test_output_expected.0 {
                test_output_found.0 = true;
            }
        }
        if data_res.watch_delay == test_output_expected.1 {
            test_output_found.1 = true;
        }
        assert_eq!(test_output_found, (true, true))
    }

    #[test]
    fn data_rec() {
        let mut test_output_found = (false, false, false);
        let test_output_expected = (
            "test/test.module.css",
            "test/recursive_test/test_r.module.css",
            0.0,
        );
        let binding = ["ztcm".to_string(), "test".to_string(), "-r".to_string()];
        let config: Config = Config::new_args(&binding).unwrap();
        let data_res = RunData::find_files(config).unwrap();
        for pathname in data_res.paths {
            if pathname == test_output_expected.0 {
                test_output_found.0 = true;
            } else if pathname == test_output_expected.1 {
                test_output_found.1 = true;
            }
        }
        if data_res.watch_delay == test_output_expected.2 {
            test_output_found.2 = true;
        }
        assert_eq!(test_output_found, (true, true, true))
    }

    #[test]
    fn data_watch() {
        let mut test_output_found = (false, false, false);
        let test_output_expected = ("test/test.module.css", 1.0, 90);
        let binding = ["ztcm".to_string(), "test".to_string(), "-w".to_string()];
        let config: Config = Config::new_args(&binding).unwrap();
        let data_res = RunData::find_files(config).unwrap();
        for pathname in data_res.paths {
            if pathname == test_output_expected.0 {
                test_output_found.0 = true;
            }
        }
        if data_res.watch_delay == test_output_expected.1 {
            test_output_found.1 = true;
        }
        if data_res.cycles_per_refresh == test_output_expected.2 {
            test_output_found.2 = true;
        }
        assert_eq!(test_output_found, (true, true, true))
    }

    #[test]
    fn data_watch_cycle() {
        let mut test_output_found = (false, false, false);
        let test_output_expected = ("test/test.module.css", 2.0, 120);
        let binding = [
            "ztcm".to_string(),
            "test".to_string(),
            "-w".to_string(),
            "2".to_string(),
            "120".to_string(),
        ];
        let config: Config = Config::new_args(&binding).unwrap();
        let data_res = RunData::find_files(config).unwrap();
        for pathname in data_res.paths {
            if pathname == test_output_expected.0 {
                test_output_found.0 = true;
            }
        }
        if data_res.watch_delay == test_output_expected.1 {
            test_output_found.1 = true;
        }
        if data_res.cycles_per_refresh == test_output_expected.2 {
            test_output_found.2 = true;
        }

        assert_eq!(test_output_found, (true, true, true))
    }

    #[test]
    fn data_outfolder() {
        let binding = [
            "ztcm".to_string(),
            "test".to_string(),
            "-o".to_string(),
            "test/testOut".to_string(),
        ];
        let config: Config = Config::new_args(&binding).unwrap();
        let _data_res = RunData::find_files(config).unwrap();

        assert_eq!(Path::new("test/testOut/test.module.css.d.ts").exists(), true)
    }
}

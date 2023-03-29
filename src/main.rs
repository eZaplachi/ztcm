use clap::Parser;
use std::{
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};
// Brings stdout flush in scope for load message
pub mod args;
mod get_file_paths;
mod parse_and_printout;
use args::Cli;
use parse_and_printout::parse_and_printout;

fn main() {
    let cli: Cli = Cli::parse();
    // println!("{cli:#?}");
    let mut _files_paths: Vec<String> = vec![];
    // let args: Vec<String> = env::args().collect();
    run_ztcm(cli)
}

fn run_ztcm(cli: Cli) {
    println!("Path: \x1b[36;1;4m{}\x1b[0m\n", cli.path);
    let mut _file_paths: Vec<String> = vec![];
    // if data.paths.len() < data.threads as usize {
    //     panic!("Error - More threads than files");
    let now = Instant::now();
    let file_paths: Vec<String> =
        get_file_paths::get_paths(cli.path.clone(), cli.pattern.clone(), cli.recursive);
    for path in &file_paths {
        println!("\x1b[34;1mFound\x1b[0m: {}", path);
    }
    let elapsed_time = now.elapsed();
    if cli.timer {
        println!("In {} microseconds.", elapsed_time.as_micros());
    }
    println!("\n");

    if cli.watch == 0.0 {
        parse_and_printout(
            &file_paths,
            cli.multithread,
            cli.camel_case,
            &cli.output,
            cli.timer,
        );
    } else {
        watch(cli, file_paths)
    }
}

fn watch(cli: Cli, paths: Vec<String>) {
    let delay = Duration::from_secs_f64(cli.watch);
    let mut file_paths: Vec<String> = paths;
    let mut load_state = 0;
    let mut _load_char = "";
    let mut load_color = 0;
    let mut i = 0;
    let path = cli.path;
    let pattern = cli.pattern;
    loop {
        parse_and_printout(
            &file_paths,
            cli.multithread,
            cli.camel_case,
            &cli.output.clone(),
            cli.timer,
        );
        thread::sleep(delay);
        // Loading icon logic
        if i % 2 == 0 {
            if load_state == 3 {
                load_state = 0;
                if load_color > 3 {
                    load_color = 0;
                } else {
                    load_color += 1;
                }
            }
            _load_char = match load_state {
                0 => "/",
                1 => "-",
                2 => "\\",
                3 => "|",
                _ => "*",
            };
            load_state += 1;
            match load_color {
                0 => print!("\r[{}]", _load_char),
                1 => print!("\r[\x1b[36m{}\x1b[0m]", _load_char),
                2 => print!("\r[\x1b[34m{}\x1b[0m]", _load_char),
                3 => print!("\r[\x1b[35m{}\x1b[0m]", _load_char),
                _ => print!("\r[{}]", _load_char),
            }
            io::stdout().flush().expect("Could not flush stdout");
        }
        if i > cli.update_after_cycles {
            i = 0;
            println!("\n\n\x1b[33mRe-Indexing Files\x1b[0m");
            file_paths = get_file_paths::get_paths(path.clone(), pattern.clone(), cli.recursive);
        }
        i += 1;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn new_args() {
//         let binding = ["ztcm".to_string(), "./test".to_string()];
//         let args = Config::new_args(&binding).unwrap();
//         assert_eq!(args.query, "./test")
//     }

//     #[test]
//     fn new_recursive_args() {
//         let binding = ["ztcm".to_string(), "./test".to_string(), "-r".to_string()];
//         let args = Config::new_args(&binding).unwrap();
//         assert_eq!(args.query, "./test");
//         assert_eq!(args.flags, vec![&"-r".to_string()])
//     }

//     #[test]
//     fn new_watch_args() {
//         let binding = [
//             "ztcm".to_string(),
//             "./test".to_string(),
//             "-w".to_string(),
//             "2".to_string(),
//         ];
//         let args = Config::new_args(&binding).unwrap();
//         assert_eq!(args.query, "./test");
//         assert_eq!(args.flags, vec![&"-w".to_string(), &"2".to_string()])
//     }

//     #[test]
//     fn test_get_data() {
//         let mut test_output_found = (false, false);
//         let test_output_expected = ("test/test.module.css", 0.0);
//         let binding = ["ztcm".to_string(), "test".to_string()];
//         let config: Config = Config::new_args(&binding).unwrap();
//         let data_res = RunData::find_files(config).unwrap();
//         for pathname in data_res.paths {
//             if pathname == test_output_expected.0 {
//                 test_output_found.0 = true;
//             }
//         }
//         if data_res.watch_delay == test_output_expected.1 {
//             test_output_found.1 = true;
//         }
//         assert_eq!(test_output_found, (true, true))
//     }

//     #[test]
//     fn test_get_data_recursive() {
//         let mut test_output_found = (false, false, false);
//         let test_output_expected = (
//             "test/test.module.css",
//             "test/recursive_test/test_r.module.css",
//             0.0,
//         );
//         let binding = ["ztcm".to_string(), "test".to_string(), "-r".to_string()];
//         let config: Config = Config::new_args(&binding).unwrap();
//         let data_res = RunData::find_files(config).unwrap();
//         for pathname in data_res.paths {
//             if pathname == test_output_expected.0 {
//                 test_output_found.0 = true;
//             } else if pathname == test_output_expected.1 {
//                 test_output_found.1 = true;
//             }
//         }
//         if data_res.watch_delay == test_output_expected.2 {
//             test_output_found.2 = true;
//         }
//         assert_eq!(test_output_found, (true, true, true))
//     }

//     #[test]
//     fn test_get_data_watch() {
//         let mut test_output_found = (false, false, false);
//         let test_output_expected = ("test/test.module.css", 1.0, 90);
//         let binding = ["ztcm".to_string(), "test".to_string(), "-w".to_string()];
//         let config: Config = Config::new_args(&binding).unwrap();
//         let data_res = RunData::find_files(config).unwrap();
//         for pathname in data_res.paths {
//             if pathname == test_output_expected.0 {
//                 test_output_found.0 = true;
//             }
//         }
//         if data_res.watch_delay == test_output_expected.1 {
//             test_output_found.1 = true;
//         }
//         if data_res.cycles_per_refresh == test_output_expected.2 {
//             test_output_found.2 = true;
//         }
//         assert_eq!(test_output_found, (true, true, true))
//     }

//     #[test]
//     fn test_get_data_watch_cycle() {
//         let mut test_output_found = (false, false, false);
//         let test_output_expected = ("test/test.module.css", 2.0, 120);
//         let binding = [
//             "ztcm".to_string(),
//             "test".to_string(),
//             "-w".to_string(),
//             "2".to_string(),
//             "120".to_string(),
//         ];
//         let config: Config = Config::new_args(&binding).unwrap();
//         let data_res = RunData::find_files(config).unwrap();
//         for pathname in data_res.paths {
//             if pathname == test_output_expected.0 {
//                 test_output_found.0 = true;
//             }
//         }
//         if data_res.watch_delay == test_output_expected.1 {
//             test_output_found.1 = true;
//         }
//         if data_res.cycles_per_refresh == test_output_expected.2 {
//             test_output_found.2 = true;
//         }
//         assert_eq!(test_output_found, (true, true, true))
//     }
// }

use glob::glob;
use regex::Regex;
use std::{error, fs, io};

pub struct Config {
    pub query: String,
    pub flags: Vec<String>,
}

impl Config {
    // Convert args to Config format
    pub fn new_args(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 5 {
            return Err("too many arguments");
        }

        let mut path = String::new();
        let mut flags = Vec::new();
        let help_message = "Usage: -h / -help:  show this help \n \n Run command:      ztcm [path('.' for cwd)] [flags] \n
        \n Flags: \n '-r': recursively search through the selected folder \n '-w {Delay* (s)}': watches for changes in file every {Delay*} second(s); *Optional - Defaults to 1s";

        for arg in args {
            if arg.contains("-h") || arg.contains("-help") {
                println!("{}", help_message);
                return Err("Help flag called");
            } else if !arg.contains("ztcm") {
                if !arg.contains("-") {
                    if path.is_empty() {
                        path = arg.clone();
                        print!("Path: {}\t\t", path);
                    } else {
                        flags.push(arg.clone());
                    }
                } else if !arg.contains(&path) {
                    flags.push(arg.clone())
                }
            }
        }

        Ok(Config {
            query: path,
            flags: flags,
        })
    }
}

pub fn run_ztcm(config: Config) -> Result<(Vec<String>, f64), Box<dyn error::Error>> {
    // Parse config and runs with flag options
    let flags = config.flags.clone();
    let flags_length = flags.clone().len();
    let mut recursive = false;
    let mut watch_delay: f64 = 0.0;
    let default_watch_delay: f64 = 1.0;
    let re = Regex::new(r"[0-9]").unwrap();

    let mut i = 0;
    for flag in flags.clone() {
        if flag == "-r" {
            print!("[Recursive]\t");
            recursive = true;
        } else if flag == "-w" {
            print!("[Watching - ");
            watch_delay = default_watch_delay;
            // If a delay number is provided set to watch_delay
            if i + 1 < flags_length {
                if re.is_match(flags[i + 1].as_str()) {
                    watch_delay = flags[i + 1].parse().unwrap();
                }
            }
            println!("Updates every {} s]", watch_delay);
        }
        i += 1;
    }

    if recursive {
        Ok((get_files_recursive(config.query.clone()), watch_delay))
    } else {
        Ok((get_files(config.query.clone()).unwrap(), watch_delay))
    }
}

fn get_files(directory: String) -> io::Result<Vec<String>> {
    let mut entries = fs::read_dir(directory)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    // The entries have now been sorted by their path.
    let mut css_files_path: Vec<String> = Vec::new();

    for entry in entries {
        let entry_string: String = entry.as_path().display().to_string();
        let files = &entry_string.ends_with(".css");
        if files == &true {
            css_files_path.push(entry_string.clone())
        }
    }
    Ok(css_files_path)
}

fn get_files_recursive(directory: String) -> Vec<String> {
    let chosen_folder = directory + "/**/*.css";

    let mut css_file_paths: Vec<String> = Vec::new();
    for entry in glob(chosen_folder.as_str()).expect("Failed to read file names recursively") {
        if let Ok(path) = entry {
            css_file_paths.push(path.display().to_string());
        } else if let Err(e) = entry {
            println!("Glob error: {:?}", e)
        }
    }
    css_file_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_file() {
        let mut test_output_found = false;
        let test_output_expected = "./test/test.module.css";

        let files = get_files("./test".to_string()).unwrap();
        for file in files {
            if file == test_output_expected {
                test_output_found = true;
            }
        }

        assert_eq!(test_output_found, true)
    }

    #[test]
    fn get_file_r() {
        let mut test_output_found = (false, false);
        let test_output_expected = (
            "test/test.module.css",
            "test/recursive_test/test_r.module.css",
        );

        let files_r = get_files_recursive("./test".to_string());
        for file in files_r {
            if file == test_output_expected.0 {
                test_output_found.0 = true;
            }
            if file == test_output_expected.1 {
                test_output_found.1 = true;
            }
        }
        assert_eq!(test_output_found, (true, true))
    }
}

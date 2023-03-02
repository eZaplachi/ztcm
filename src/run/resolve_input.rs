use regex::Regex;
use std::error;
mod find_files;

pub struct Config {
    pub query: String,
    pub flags: Vec<String>,
}

impl Config {
    // Convert args to Config format
    pub fn new_args(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 6 {
            return Err("too many arguments");
        }

        let mut path = String::new();
        let mut flags = Vec::new();
        let help_message = "Usage: -h / -help:  show this help \n \n Run command:      ztcm [path('.' for cwd)] [flags] \n
        \n Flags: \n '-r': recursively search through the selected folder \n '-w {Delay* (s)}': watches for changes in file every {Delay*} second(s); *Optional - Defaults to 1s \n '-c': converts class and id names from pascal case to camel case";

        for arg in args {
            if arg.contains("-h") || arg.contains("-help") {
                println!("{}", help_message);
                return Err("Help flag called");
            } else if !arg.contains("ztcm") {
                if !arg.contains('-') {
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

        Ok(Config { query: path, flags })
    }

    pub fn get_files(config: Config) -> Result<(Vec<String>, bool, f64), Box<dyn error::Error>> {
        // Parse config and runs with flag options
        let flags = config.flags.clone();
        let flags_length = flags.len();
        let mut recursive = false;
        let mut watch_delay: f64 = 0.0;
        let default_watch_delay: f64 = 1.0;
        let mut camel_case_flag: bool = false;
        let re = Regex::new(r"[0-9]").unwrap();

        for (i, flag) in flags.clone().into_iter().enumerate() {
            if flag == "-r" {
                print!("[Recursive]\t");
                recursive = true;
            } else if flag == "-c" {
                print!("[Pascal-Case --> camelCase]");
                camel_case_flag = true;
            } else if flag == "-w" {
                print!("[Watching - ");
                watch_delay = default_watch_delay;
                // If a delay number is provided set to watch_delay
                if i + 1 < flags_length && re.is_match(flags[i + 1].as_str()) {
                    watch_delay = flags[i + 1].parse().unwrap();
                }
                println!("Updates every {} s]", watch_delay);
            }
        }

        if recursive {
            Ok((
                find_files::get_files_recursive(config.query),
                camel_case_flag,
                watch_delay,
            ))
        } else {
            Ok((
                find_files::get_files(config.query).unwrap(),
                camel_case_flag,
                watch_delay,
            ))
        }
    }
}

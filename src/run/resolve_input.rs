use glob::glob;
use regex::Regex;

pub struct Config<'a> {
    pub query: String,
    pub flags: Vec<&'a String>,
}

pub struct RunData {
    pub paths: Vec<String>,
    pub cc_flag: bool,
    pub kc_flag: bool,
    pub out_dir: String,
    pub watch_delay: f64,
    pub cycles_per_refresh: i32,
}

impl Config<'_> {
    // Convert args to Config format
    pub fn new_args(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 7 {
            return Err("too many arguments");
        }
        let mut path = String::new();
        let mut flags = Vec::new();
        let help_message = "Usage: '-h' / '-help': Show this help \n \n Run command:      ztcm [path('.' for cwd)] [flags] \n
        \n Flags: \n '-r': Recursively search through the selected folder \n '-w [*Cycle Delay (s)] [*Cycles/Refresh]': Watches for changes in file every cycle; *Optional - Defaults to [1s] [90cycles/refresh]
        \n '-c': Converts class and id names from kebab-case to camelCase \n '-k': Converts from camelCase to kebab-case";

        for arg in args {
            if arg.contains("-h") || arg.contains("-help") || arg.contains("--help") {
                println!("{}", help_message);
                return Err("Help flag called");
            } else if !arg.contains("ztcm") {
                if !arg.contains('-') {
                    if path.is_empty() {
                        path = arg.clone();
                        print!("Path: {}\t\t", path);
                    } else {
                        flags.push(arg);
                    }
                } else if !arg.contains(&path) {
                    flags.push(arg)
                }
            }
        }
        Ok(Config { query: path, flags })
    }
}

impl RunData {
    pub fn find_files(config: Config) -> Result<RunData, &'static str> {
        // Parse config and runs with flag options
        let flags = config.flags.clone();
        let flags_length = flags.len();
        let mut recursive = false;
        let mut watch_delay: f64 = 0.0;
        let default_watch_delay: f64 = 1.0;
        let mut cycles_per_refresh: i32 = 90;
        let mut camel_case_flag: bool = false;
        let mut kebab_case_flag: bool = false;
        let mut out_dir: String = String::new();
        let re_num = Regex::new(r"[0-9]").unwrap();
        let re_word = Regex::new(r"[\w*]").unwrap();

        for (i, flag) in flags.clone().into_iter().enumerate() {
            if flag == "-r" {
                print!("[Recursive]\t");
                recursive = true;
            } else if flag == "-c" {
                print!("[kebab-case --> camelCase]");
                camel_case_flag = true;
            } else if flag == "-k" {
                print!("[camelCase --> kebab-case]");
                kebab_case_flag = true;
            } else if flag == "-w" {
                print!("[Watching - ");
                // If a delay number is provided set to watch_delay
                if i + 1 < flags_length && re_num.is_match(flags[i + 1].as_str()) {
                    watch_delay = flags[i + 1].parse().unwrap();
                } else {
                    watch_delay = default_watch_delay;
                }
                if i + 2 < flags_length && re_num.is_match(flags[i + 2].as_str()) {
                    cycles_per_refresh = flags[i + 2].parse().unwrap();
                }
            } else if flag == "-o" {
                if i + 1 < flags_length && re_word.is_match(flags[i + 1].as_str()) {
                    out_dir = flags[i + 1].to_string();
                } else {
                    return Err("No output directory specified");
                }
            }
        }
        if camel_case_flag && kebab_case_flag {
            panic!("Can't have both the Camel and Kebab case flags called at once")
        }
        println!(
            "Updates cycle every {} s, Refresh every {} cycles]",
            watch_delay, cycles_per_refresh
        );
        if recursive {
            Ok(RunData {
                paths: get_files_recursive(config.query),
                cc_flag: camel_case_flag,
                kc_flag: kebab_case_flag,
                out_dir,
                watch_delay,
                cycles_per_refresh,
            })
        } else {
            Ok(RunData {
                paths: get_files(config.query),
                cc_flag: camel_case_flag,
                kc_flag: kebab_case_flag,
                out_dir,
                watch_delay,
                cycles_per_refresh,
            })
        }
    }
}

fn get_files(directory: String) -> Vec<String> {
    let chosen_folder = directory + "/*.module.css";

    let mut css_file_paths: Vec<String> = Vec::new();
    for entry in glob(chosen_folder.as_str()).expect("Failed to read file names") {
        if let Ok(path) = entry {
            css_file_paths.push(path.display().to_string());
        } else if let Err(e) = entry {
            println!("Glob error: {:?}", e)
        }
    }
    css_file_paths
}

fn get_files_recursive(directory: String) -> Vec<String> {
    let chosen_folder = directory + "/**/*.module.css";

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
    #[should_panic]
    fn camel_kebab_case() {
        let _res = RunData::find_files(Config {
            query: "./test".to_string(),
            flags: vec![&"-c".to_string(), &"-k".to_string()],
        });
    }

    #[test]
    fn get_file() {
        let mut test_output_found = false;
        let test_output_expected = "test/test.module.css";

        let files = get_files("./test".to_string());
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

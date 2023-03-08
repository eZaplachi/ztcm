use glob::glob;
use regex::Regex;

pub struct Config<'a> {
    pub query: String,
    pub flags: Vec<&'a String>,
    defaults: Defaults,
}

pub struct RunData {
    pub paths: Vec<String>,
    pub cc_flag: bool,
    pub kc_flag: bool,
    pub out_dir: String,
    pub watch_delay: f64,
    pub cycles_per_refresh: i32,
    pub threads: i32,
}

struct Defaults {
    pattern: String,
    delay: f64,
    re_index: i32,
    threads: i32,
}

fn get_defaults() -> Defaults {
    Defaults {
        pattern: ".module.css".to_string(),
        delay: 1.0,
        re_index: 90,
        threads: 2,
    }
}

impl Config<'_> {
    // Convert args to Config format
    pub fn new_args(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 12 {
            return Err("too many arguments");
        }
        let mut path = String::new();
        let mut flags = Vec::new();
        let defaults = get_defaults();
        for arg in args {
            if arg.contains("-h") || arg.contains("-help") || arg.contains("--help") {
                println!("Usage: '-h' / '-help': Show this help \n
                    \n Run command:      ztcm [path('.' for cwd)] [flags] \n
                    \n Flags:
                    \n '-r': Recursively search through the selected folder 
                    \n '-w [*Cycle Delay (s)] [*Cycles/Refresh]': Watches for changes in file every cycle; *Optional - Defaults to [{}s] [Every {}cycles/refresh]
                    \n '-c': Converts class and id names from kebab-case to camelCase for .d.ts files
                    \n '-k': Converts from camelCase to kebab-case
                    \n '-o [Out-dir]'': Changes output directory
                    \n '-p [Pattern]': Choose pattern to search - Defaults to [{}]
                    \n '-m [Threads]': Enable multi-threaded mode - Defaults to [{} threads]", defaults.delay, defaults.re_index, defaults.pattern, defaults.threads);

                return Err("Help flag called");
            } else if !arg.contains("ztcm") {
                if !arg.contains('-') {
                    if path.is_empty() {
                        path = arg.clone();
                        print!("Path: \x1b[36;1;4m{}\x1b[0m\t", path);
                    } else {
                        flags.push(arg);
                    }
                } else if !arg.contains(&path) {
                    flags.push(arg)
                }
            }
        }
        Ok(Config {
            query: path,
            flags,
            defaults,
        })
    }
}

impl RunData {
    pub fn find_files(config: Config) -> Result<RunData, &'static str> {
        // Parse config and runs with flag options
        let flags = config.flags;
        let flags_length = flags.len();
        let mut recursive = false;
        let mut watch_delay: f64 = 0.0;
        let mut cycles_per_refresh: i32 = 0;
        let mut threads: i32 = 1;
        let mut camel_case_flag: bool = false;
        let mut kebab_case_flag: bool = false;
        let mut out_dir: String = String::new();
        let mut pattern: String = config.defaults.pattern;
        let re_num = Regex::new(r"[0-9]").unwrap();
        let re_word = Regex::new(r"[\w*]").unwrap();
        let custom_tab = "    ";

        for (i, flag) in flags.clone().into_iter().enumerate() {
            match flag.as_str() {
                "-r" => {
                    print!("[Recursive]{}", custom_tab);
                    recursive = true;
                }
                "-c" => {
                    print!("[kebab-case --> camelCase.d.ts]{}", custom_tab);
                    camel_case_flag = true;
                }
                "-k" => {
                    print!("[camelCase --> kebab-case]{}", custom_tab);
                    kebab_case_flag = true;
                }
                "-o" => {
                    if i + 1 < flags_length && re_word.is_match(flags[i + 1].as_str()) {
                        out_dir = flags[i + 1].to_string();
                        print!("[Out-dir {}]{}", out_dir, custom_tab);
                    } else {
                        return Err("No output directory specified");
                    }
                }
                "-p" => {
                    if i + 1 < flags_length && re_word.is_match(flags[i + 1].as_str()) {
                        pattern = flags[i + 1].to_string();
                        print!("[Pattern {}]{}", pattern, custom_tab);
                    } else {
                        return Err("No pattern specified");
                    }
                }
                "-m" => {
                    print!("[Multithreaded]{}", custom_tab);
                    // If a delay number is provided set to watch_delay
                    if i + 1 < flags_length && re_num.is_match(flags[i + 1].as_str()) {
                        threads = flags[i + 1]
                            .parse()
                            .expect("Error getting number of threads");
                    } else {
                        threads = config.defaults.threads;
                    }
                }

                "-w" => {
                    print!("[Watching - ");
                    // If a delay number is provided set to watch_delay
                    if i + 1 < flags_length && re_num.is_match(flags[i + 1].as_str()) {
                        watch_delay = flags[i + 1].parse().expect("Error getting watch delay");
                    } else {
                        watch_delay = config.defaults.delay;
                    }
                    if i + 2 < flags_length && re_num.is_match(flags[i + 2].as_str()) {
                        cycles_per_refresh = flags[i + 2]
                            .parse()
                            .expect("Error getting cycles per refresh");
                    } else {
                        cycles_per_refresh = config.defaults.re_index;
                    }
                    println!(
                        "Updates cycle every {} s, Refresh every {} cycles]",
                        watch_delay, cycles_per_refresh
                    );
                }
                _ => {}
            }
        }
        if camel_case_flag && kebab_case_flag {
            panic!("Can't have both the Camel and Kebab case flags called at once")
        }
        if recursive {
            Ok(RunData {
                paths: get_files_recursive(config.query, pattern),
                cc_flag: camel_case_flag,
                kc_flag: kebab_case_flag,
                out_dir,
                watch_delay,
                cycles_per_refresh,
                threads,
            })
        } else {
            Ok(RunData {
                paths: get_files(config.query, pattern),
                cc_flag: camel_case_flag,
                kc_flag: kebab_case_flag,
                out_dir,
                watch_delay,
                cycles_per_refresh,
                threads,
            })
        }
    }
}

fn get_files(directory: String, pattern: String) -> Vec<String> {
    let chosen_folder = directory + "/*" + &pattern;

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

fn get_files_recursive(directory: String, pattern: String) -> Vec<String> {
    let chosen_folder = directory + "/**/*" + &pattern;

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
        let defaults: Defaults = get_defaults();
        let _res = RunData::find_files(Config {
            query: "./test".to_string(),
            flags: vec![&"-c".to_string(), &"-k".to_string()],
            defaults,
        });
    }

    #[test]
    fn get_file() {
        let defaults: Defaults = get_defaults();
        let mut test_output_found = false;
        let test_output_expected = "test/test.module.css";

        let files = get_files("./test".to_string(), defaults.pattern);
        for file in files {
            if file == test_output_expected {
                test_output_found = true;
            }
        }

        assert_eq!(test_output_found, true)
    }

    #[test]
    fn get_file_rec() {
        let defaults: Defaults = get_defaults();
        let mut test_output_found = (false, false);
        let test_output_expected = (
            "test/test.module.css",
            "test/recursive_test/test_r.module.css",
        );

        let files_r = get_files_recursive("./test".to_string(), defaults.pattern);
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

    #[test]
    fn get_file_path() {
        let mut test_output_found = false;
        let test_output_expected = "test/test.module.scss";

        let files = get_files("./test".to_string(), ".scss".to_string());
        for file in files {
            if file == test_output_expected {
                test_output_found = true;
            }
        }

        assert_eq!(test_output_found, true)
    }
}

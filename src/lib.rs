use glob::glob;
use std::{error, fs, io};

pub struct Config {
    pub query: String,
    pub flags: Vec<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 5 {
            return Err("too many arguments");
        }

        let mut stripped_args = Vec::new();
        for arg in args {
            if !arg.contains("ztc") {
                stripped_args.push(arg.clone());
            }
        }

        let mut path = String::new();
        let mut flags = Vec::new();

        let help_message = "Usage: -h / -help:  show this help \n \n Run command:      ztcm [path('.' for cwd)] [flags] \n\n Flags: \n '-r': recursively search through the selected folder \n ";

        for arg in stripped_args {
            if !arg.contains("-") {
                if path.is_empty() {
                    path = arg;
                    println!("Path: {}", path);
                } else {
                    println!("More than one path found");
                    return Err("Too many paths");
                }
            } else if arg.contains("-h") || arg.contains("-help") {
                println!("{}", help_message);
                return Err("Help flag called");
            } else {
                flags.push(arg)
            }
        }

        Ok(Config {
            query: path,
            flags: flags,
        })
    }
}

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn error::Error>> {
    // Parse flags and runs with options
    if config.flags.contains(&"-r".to_string()) {
        Ok(get_files_recursive(config.query.clone()))
    } else {
        Ok(get_files(config.query.clone()).unwrap())
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
    let mut css_files: Vec<String> = Vec::new();

    for entry in entries {
        let entry_string: String = entry.as_path().display().to_string();
        let files = &entry_string.ends_with(".css");
        if files == &true {
            css_files.push(entry_string.clone())
        }
    }
    Ok(css_files)
}

fn get_files_recursive(directory: String) -> Vec<String> {
    let chosen_folder = directory + "/**/*.css";

    let mut css_file_paths: Vec<String> = Vec::new();
    for entry in glob(chosen_folder.as_str()).expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            css_file_paths.push(path.display().to_string());
        } else if let Err(e) = entry {
            println!("Glob error: {:?}", e)
        }

    }
    css_file_paths
}

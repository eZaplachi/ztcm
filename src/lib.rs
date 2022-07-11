use glob::glob;
use std::{error, fs, io};

pub struct Config {
    pub query: String,
    pub flag: String,
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
            if arg != "target/debug/ztcm" || arg != "ztc" {
                stripped_args.push(arg.clone());
            }
        };


        for arg in stripped_args {
            if !arg.contains("-") {
                println!("Query folder: {}", arg);
            }
        }


        let q = args[1].clone();


        if !q.contains("-") && args.len() == 2 {
            Ok(Config {
                query: q,
                flag: String::new(),
            })
        } else if q.contains("-h") || q.contains("-help") && args.len() == 2 {
            println!("Usage: -h / -help:  show this help \n \n Run command:      ztcm [folder('.' for cwd)] [flags] \n
             \n Flags: \n '-r': recursively search through the selected folder \n ");
            Err("help")
        } else {

            Ok(Config {

                query: q,
                flag: args[2].clone(),
            })
        }

        // Ok(Config { query, flag })
    }
}

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn error::Error>> {
    let set_options = config.flag;

    if set_options == "-r" {
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
            // println!("{:?}", path.display())
        } else if let Err(e) = entry {
            println!("Glob error: {:?}", e)
        }

        // match entry {
            //     Ok(path) => println!("{:?}", path.display()),
            //     Err(e) => println!("{:?}", e),
            // }
        }
        css_file_paths
}

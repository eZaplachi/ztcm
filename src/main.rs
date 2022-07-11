// use lazy_static::lazy_static;
// use regex::RegexSet;
// use std::{collections::HashMap, fs,};
use std::{env, process};
use ztcm::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let css_path_names = ztcm::run(config).unwrap_or_else(|err| {
        println!("Problem parsing file paths: {}", err);
        process::exit(1);
    });


    for path_name in css_path_names {
        println!("Found file: {:?}", path_name);
    }

    // for x in css_data {
    //     println!("{}", x);
    // }
    // let contents = fs::read_to_string(config.filename)
    //     .expect("Something went wrong reading the file");
}

// fn __search_files(text: &str) -> (bool, bool) {
//     lazy_static! {
//         static ref RE: RegexSet = RegexSet::new(&[r".\.{1,}?", r"css$"]).unwrap();
//     }

//     (RE.matches(text).matched(0), RE.matches(text).matched(1))
// }

// fn __parse_file_paths(map: HashMap<String, &str>) -> (Vec<String>, Vec<String>) {
//     let mut folders_in_cd: Vec<String> = Vec::new();
//     let mut files_in_cd: Vec<String> = Vec::new();
//     for (key, value) in map {
//         if value == "folder" {
//             folders_in_cd.push(key.clone());
//         }
//         if value == "css" {
//             files_in_cd.push(key.clone());
//         }
//     }

//     (folders_in_cd, files_in_cd)
// }

// fn __parse_files(file_paths: (Vec<String>, Vec<String>)) -> Vec<String> {
//     let mut css_data = Vec::new();
//     for files in file_paths.1 {
//         let content = fs::read_to_string(files).expect("Can't read file");
//         css_data.push(content);
//     }
//     css_data
// }

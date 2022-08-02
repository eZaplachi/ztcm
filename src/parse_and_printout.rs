use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::{fs, path, thread, time};

pub struct ParseRes {
    // file_res: Vec<String>,
}
// -> Result<Parse_res, &'static str>
impl ParseRes {
    pub fn parse_and_print_out(path_names: Vec<String>, watch_delay: f64) {
        for path in path_names.clone() {
            println!("\nFound: {}", path);
        }
        if watch_delay != 0.0 {
            let delay = time::Duration::from_secs_f64(watch_delay);
            loop {
                parse_files(&path_names);

                thread::sleep(delay);
            }
        } else {
            parse_files(&path_names);
        }
    }
}

fn find_classes_or_ids(text: &str) -> HashSet<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)^[\.\#]\w*").unwrap();
    }
    RE.find_iter(text).map(|mat| mat.as_str()).collect()
}

fn find_declarations(text: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"readonly '\w*': \w*").unwrap();
    }
    RE.find_iter(text).map(|mat| mat.as_str()).collect()
}

fn parse_files(path_names: &[String]) {
    for path in path_names {
        let (data_vec, outfile_name) = get_file_data(path);
        print_files(data_vec, outfile_name);
    }
}

fn get_file_data(path: &String) -> (Vec<String>, String) {
    let mut out_names = Vec::new();
    let re = Regex::new(r"[\.\#]").unwrap();
    let outfile_path = format!("{}.d.ts", path);
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let names = find_classes_or_ids(&contents);
    for name in names {
        let parsed_name = re.replace_all(name, "");
        let out_name = format!("readonly '{}': string;", parsed_name);
        out_names.push(out_name)
        // println!("{:?}", out_name)
    }

    // println!("{:?}", out_names);
    (out_names, outfile_path)
}

fn print_files(data_vec: Vec<String>, outfile_name: String) {
    let path_exists = path::Path::new(&outfile_name).exists();
    let mut _outfile_data = String::new();
    let mut outfile_vec_set = Vec::new();
    let mut matching_value = false;
    let mut print_out = false;
    if !path_exists {
        println!("Creating file: {}", outfile_name);
    } else {
        _outfile_data = fs::read_to_string(&outfile_name).unwrap();
        outfile_vec_set = find_declarations(&_outfile_data);
        // println!("{:?}", outfile_vec_set);
    }

    let mut intermediate_string = String::new();
    for data in data_vec {
        intermediate_string = format!("{} {}\n", intermediate_string, data);
        for line in &outfile_vec_set {
            let formatted_line = format!("{};", line);
            // println!("{}  :  {:?}\n", data, format!("{};", line));
            if data == formatted_line {
                // println!("{}", data)
                matching_value = true;
            }
        }
        if !matching_value {
            print_out = true;
        }
        matching_value = false;
    }

    let data_string = format!(
        "declare const styles: {{\n{}\n}};\nexport = styles;",
        intermediate_string
    );
    // println!("{}", data_string);

    // println!("{} \n {}", data_string, outfile_data);
    if print_out {
        fs::write(outfile_name.clone(), data_string)
            .expect("An Error creating deceleration file occurred");
        println!("Wrote to file: {}", outfile_name)
    }
    // else {
    //     println!("Finished")
    // }
    // println!("{:?}", path_exists)
}

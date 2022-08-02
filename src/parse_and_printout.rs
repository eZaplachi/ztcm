use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, path, thread, time, io};
use std::io::prelude::*;

pub struct ParseRes {}
impl ParseRes {
    pub fn parse_and_print_out(path_names: Vec<String>, watch_delay: f64) {
        for path in path_names.clone() {
            println!("\nFound: {}", path);
        }
        if watch_delay != 0.0 {
            let delay = time::Duration::from_secs_f64(watch_delay);
            let mut load_state = 0;
            let mut _load_char = "";
            let mut i = 0;
            loop {
                parse_files(&path_names);

                thread::sleep(delay);

                if i > 3 {
                    i = 0;
                    if load_state == 3 {
                        load_state = 0;
                    }
                    _load_char = match load_state {
                        0 => "/",
                        1 => "-",
                        2 => "\\",
                        3 => "|",
                        _ => "*"
                    };


                    load_state += 1;
                    print!("\r[{}]", _load_char);
                    io::stdout().flush().ok().expect("Could not flush stdout");
                }
                i += 1;
            }
        } else {
            parse_files(&path_names);
        }
    }
}

// Regex functions
fn find_classes_or_ids(text: &str) -> Vec<&str> {
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

// Logic function
fn parse_files(path_names: &[String]) {
    for path in path_names {
        let (data_vec, outfile_name) = get_file_data(path);
        print_files(data_vec, outfile_name);
    }
}

// Helper logic functions
fn get_file_data(path: &String) -> (Vec<String>, String) {
    let outfile_path = format!("{}.d.ts", path);
    let contents = fs::read_to_string(path).expect("Something went wrong reading the .css file");
    let mut out_names = Vec::new();
    let re = Regex::new(r"[\.\#]").unwrap();

    let names = find_classes_or_ids(&contents);
    for name in names {
        let parsed_name = re.replace_all(name, "");
        let out_name = format!("readonly '{}': string;", parsed_name);
        out_names.push(out_name)
    }
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
        _outfile_data = fs::read_to_string(&outfile_name).expect("Something went wrong reading the .d.ts file");
        outfile_vec_set = find_declarations(&_outfile_data);
    }

    let mut intermediate_string = String::new();
    for data in data_vec {
        intermediate_string = format!("{} {}\n", intermediate_string, data);
        for line in &outfile_vec_set {
            let formatted_line = format!("{};", line);
            if data == formatted_line {
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

    if print_out {
        fs::write(outfile_name.clone(), data_string)
            .expect("An Error creating deceleration file occurred");
        println!("\rWrote to file: {}", outfile_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_c_or_id() {
        let class_or_id = find_classes_or_ids(".testClass \n#testId");
        let class_or_id_expected = [".testClass", "#testId"];
        assert_eq!(class_or_id, class_or_id_expected)
    }

    #[test]
    fn find_decls() {
        let declarations = find_declarations("readonly 'test': string;\n readonly 'test2': string;");
        let declarations_expected = ["readonly 'test': string", "readonly 'test2': string"];
        assert_eq!(declarations, declarations_expected)
    }
}
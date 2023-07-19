use crate::parse_and_printout::print_multithreaded;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, fs, path::Path};

pub fn print_files(
    data_set: HashSet<String>,
    outfile_name: String,
    is_multithreaded: bool,
    thread_num: i32,
) {
    let path_exists = Path::new(&outfile_name).exists();
    let mut _outfile_data = String::new();
    let mut outfile_set = HashSet::new();
    let mut matching_value = false;
    let mut print_out = false;
    if !path_exists {
        if !data_set.len() == 0 {
            let print_text = format!("\x1b[33;1mCreating\x1b[0m: {}", outfile_name);
            if is_multithreaded {
                print_multithreaded(print_text, thread_num)
            } else {
                println!("{}", print_text)
            }
        }
    } else {
        _outfile_data = fs::read_to_string(&outfile_name)
            .expect("Something went wrong reading the .css.ts file");
        outfile_set = find_declarations(&_outfile_data);
    }

    let mut intermediate_string = String::new();
    for data in data_set {
        intermediate_string = format!("{} {}\n", intermediate_string, data);
        for line in &outfile_set {
            if &&data == line {
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
        let print_text = format!("\x1b[32;1mWriting\x1b[0m to: {}", outfile_name);
        if is_multithreaded {
            print_multithreaded(print_text, thread_num)
        } else {
            println!("{}", print_text)
        }
    }
}

fn find_declarations(text: &str) -> HashSet<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"readonly '\S*': \w*;").unwrap();
    }
    RE.find_iter(text).map(|mat| mat.as_str()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let data_set = HashSet::from([
            "readonly 'test': string;".to_string(),
            "readonly 'test2': string;".to_string(),
        ]);
        let output_name = "./test/test_print.module.css.ts".to_string();
        print_files(data_set, output_name.clone(), false, 1);
        assert_eq!(Path::new(&output_name).exists(), true)
    }

    #[test]
    fn test_find_declarations() {
        let declarations =
            find_declarations("readonly 'test': string;\n readonly 'test2': string;");
        let declarations_expected =
            HashSet::from(["readonly 'test': string;", "readonly 'test2': string;"]);
        assert_eq!(declarations, declarations_expected)
    }
}

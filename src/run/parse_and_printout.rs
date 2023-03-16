use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, fs, path::Path};
pub mod parse;
use parse::{parse_file_data, ModFlags};

pub fn parse_and_print(path_names: &[String], mod_flags: ModFlags, thread_num: i32) {
    for path in path_names {
        let (data_vec, outfile_name) = parse_file_data(path, &mod_flags);
        if !outfile_name.contains("global") {
            print_files(data_vec, outfile_name, thread_num);
        }
    }
}

fn print_files(data_set: HashSet<String>, outfile_name: String, thread_num: i32) {
    let path_exists = Path::new(&outfile_name).exists();
    let mut _outfile_data = String::new();
    let mut outfile_set = HashSet::new();
    let mut matching_value = false;
    let mut print_out = false;
    if !path_exists {
        if !data_set.len() == 0 {
            println!(
                "\x1b[33;1mCreating\x1b[0m(T{}): {}",
                thread_num, outfile_name
            );
        }
    } else {
        _outfile_data =
            fs::read_to_string(&outfile_name).expect("Something went wrong reading the .d.ts file");
        outfile_set = find_declarations(&_outfile_data);
    }

    let mut intermediate_string = String::new();
    for data in data_set {
        intermediate_string = format!("{} {}\n", intermediate_string, data);
        for line in &outfile_set {
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
        println!(
            "\x1b[32;1mWriting\x1b[0m(T{}): {}\n",
            thread_num, outfile_name
        )
    }
}

fn find_declarations(text: &str) -> HashSet<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"readonly '\S*': \w*").unwrap();
    }
    RE.find_iter(text).map(|mat| mat.as_str()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_declarations() {
        let declarations =
            find_declarations("readonly 'test': string;\n readonly 'test2': string;");
        let declarations_expected =
            HashSet::from(["readonly 'test': string", "readonly 'test2': string"]);
        assert_eq!(declarations, declarations_expected)
    }

    #[test]
    fn parse_and_print_files() {
        let paths_expected = [
            "./test/test.module.css",
            "./test/recursive_test/test_r.module.css",
        ];
        parse_and_print(
            &[paths_expected[0].to_string(), paths_expected[1].to_string()],
            ModFlags {
                camel_case_flag: false,
                out_dir: &String::new(),
            },
            1,
        );
        let outputs_expected: Vec<String> = paths_expected
            .into_iter()
            .map(|path| path.to_owned() + ".d.ts")
            .collect();
        let path_exists = (
            Path::new(&outputs_expected[0]).exists(),
            Path::new(&outputs_expected[1]).exists(),
        );
        assert_eq!(path_exists, (true, true))
    }

    #[test]
    fn parse_and_print_outdir() {
        let paths_expected = [
            "./test/test.module.css",
            "./test/test_outdir/test.module.css.d.ts",
        ];
        parse_and_print(
            &[paths_expected[0].to_string()],
            ModFlags {
                camel_case_flag: false,
                out_dir: &"test/test_outdir".to_string(),
            },
            1,
        );
        assert_eq!(Path::new(paths_expected[1]).exists(), true)
    }

    #[should_panic]
    #[test]
    fn print_error_file() {
        let paths_expected = ["./test/error.module.css", "./test/error.module.css.d.ts"];
        parse_and_print(
            &[paths_expected[0].to_string()],
            ModFlags {
                camel_case_flag: false,
                out_dir: &String::new(),
            },
            1,
        );
        assert_eq!(Path::new(paths_expected[1]).exists(), true)
    }

    #[should_panic]
    #[test]
    fn print_empty_file() {
        let paths_expected = ["./test/empty.module.css", "./test/empty.module.css.d.ts"];
        parse_and_print(
            &[paths_expected[0].to_string()],
            ModFlags {
                camel_case_flag: false,
                out_dir: &"./test/test_outdir".to_string(),
            },
            1,
        );
        assert_eq!(Path::new(paths_expected[1]).exists(), true)
    }
}

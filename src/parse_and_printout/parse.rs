use std::{
    collections::HashSet,
    fs::{self, create_dir_all},
    path::Path,
};

mod reserved_text;
use crate::parse_and_printout::str_ext::StrExt;

pub struct ModFlags<'c> {
    pub camel_case_flag: bool,
    pub out_dir: &'c String,
}

pub fn parse_file_data(path: &String, mod_flags: &ModFlags) -> (HashSet<String>, String) {
    let mut __outfile_name = String::new();
    if !mod_flags.out_dir.is_empty() {
        if !Path::new(mod_flags.out_dir).exists() {
            create_dir_all(mod_flags.out_dir).expect("Couldn't create output directory");
        }
        let mod_path = mod_flags.out_dir.to_owned()
            + "/"
            + path.split('/').last().expect("Error parsing file name");
        __outfile_name = format!("{}.ts", mod_path);
    } else {
        __outfile_name = format!("{}.ts", path);
    }
    let contents = fs::read_to_string(path).expect("Something went wrong reading the .css file");
    let mut out_names = HashSet::new();
    let mut __out_name = String::new();
    let san_contents = contents.as_str().remove_comments();
    let names = san_contents.get_classes_or_ids();
    for name in names {
        let split_names = name.split_classes_ids();
        for split_name in split_names {
            let parsed_name = split_name.remove_modifiers();
            if !check_reserved(parsed_name.to_string()) {
                if mod_flags.camel_case_flag {
                    let camel_name = parsed_name.camel_case_converter();
                    __out_name = format_line(camel_name);
                    out_names.insert(__out_name);
                } else {
                    __out_name = format_line(parsed_name.to_string());
                    out_names.insert(__out_name);
                }
            }
        }
    }
    (out_names, __outfile_name)
}

fn format_line(name: String) -> String {
    format!("readonly '{}': string;", name)
}

fn check_reserved(word: String) -> bool {
    let res_vec = reserved_text::ts_reserved_words();
    for res in res_vec {
        if word == res {
            return true;
        }
    }
    false
}

// extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;

    #[test]
    fn check_reserved_names() {
        let res_true = check_reserved("any".to_string());
        let res_false = check_reserved("test".to_string());
        assert_eq!((res_true, res_false), (true, false))
    }

    #[test]
    fn test_parse_file_data() {
        let file_data_expected: (HashSet<String>, String) = (
            HashSet::from([
                "readonly 'test-class': string;".to_string(),
                "readonly 'test-id': string;".to_string(),
                "readonly 'test': string;".to_string(),
                "readonly 'split-test': string;".to_string(),
            ]),
            "./test/test.module.css.ts".to_string(),
        );
        let file_data = parse_file_data(
            &"./test/test.module.css".to_string(),
            &ModFlags {
                camel_case_flag: false,
                out_dir: &String::new(),
            },
        );
        let mut diff = false;
        let test: Vec<&String> = file_data.0.difference(&file_data_expected.0).collect();
        if !test.is_empty() {
            diff = true
        }
        assert_eq!((diff, file_data.1), (false, file_data_expected.1))
    }

    // #[bench]
    // fn bench_parse_file_data(b: &mut Bencher) {
    //     b.iter(|| {
    //         parse_file_data(
    //             &"./test/test.module.css".to_string(),
    //             &ModFlags {
    //                 camel_case_flag: false,
    //                 out_dir: &String::new(),
    //             },
    //         )
    //     })
    // }

    #[test]
    fn test_get_file_recursive_data() {
        let file_data_expected: (HashSet<String>, String) = (
            HashSet::from([
                "readonly 'R-test-Class': string;".to_string(),
                "readonly 'RtestId': string;".to_string(),
            ]),
            "./test/recursive_test/test_r.module.css.ts".to_string(),
        );
        let file_data = parse_file_data(
            &"./test/recursive_test/test_r.module.css".to_string(),
            &ModFlags {
                camel_case_flag: false,
                out_dir: &String::new(),
            },
        );
        assert_eq!(file_data, file_data_expected)
    }
}

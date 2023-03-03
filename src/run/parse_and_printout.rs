use lazy_static::lazy_static;
use regex::Regex;
use std::{fs::{self, create_dir_all}, path::Path};

pub struct ModFlags<'c> {
    pub camel_case_flag: bool,
    pub kebab_case_flag: bool,
    pub out_dir: &'c String,
}

pub fn parse_and_print(path_names: &[String], mod_flags: ModFlags) {
    for path in path_names {
        let (data_vec, outfile_name) = get_file_data(path, &mod_flags);
        print_files(data_vec, outfile_name);
    }
}

fn get_file_data(path: &String, mod_flags: &ModFlags) -> (Vec<String>, String) {
    let mut __outfile_name = String::new();
    if !mod_flags.out_dir.is_empty() {
        if !Path::new(mod_flags.out_dir).exists() {
            create_dir_all(mod_flags.out_dir).expect("Couldn't create output directory");
        }
        let mod_path = mod_flags.out_dir.to_owned() + "/" + path.split('/').last().expect("Error parsing file name");
        __outfile_name = format!("{}.d.ts", mod_path);
    } else {
        __outfile_name = format!("{}.d.ts", path);
    }
    let contents = fs::read_to_string(path).expect("Something went wrong reading the .css file");
    let mut out_names = Vec::new();
    let mut __out_name = String::new();
    let names = find_classes_or_ids(&contents);
    for name in names {
        let parsed_name = remove_modifiers(name);
        if mod_flags.camel_case_flag {
            let camel_name = camel_case_converter(&parsed_name);
            __out_name = format_line(camel_name);
            out_names.push(__out_name)
        } else if mod_flags.kebab_case_flag {
            let kebab_name = kebab_case_converter(&parsed_name);
            __out_name = format_line(kebab_name);
            out_names.push(__out_name)
        } else {
            __out_name = format_line(parsed_name.to_string());
            out_names.push(__out_name)
        }
    }
    (out_names, __outfile_name)
}

fn format_line(name: String) -> String {
    format!("readonly '{}': string;", name)
}

fn print_files(data_vec: Vec<String>, outfile_name: String) {
    let path_exists = Path::new(&outfile_name).exists();
    let mut _outfile_data = String::new();
    let mut outfile_vec_set = Vec::new();
    let mut matching_value = false;
    let mut print_out = false;
    if !path_exists {
        println!("Creating file: {}", outfile_name);
    } else {
        _outfile_data =
            fs::read_to_string(&outfile_name).expect("Something went wrong reading the .d.ts file");
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

fn find_classes_or_ids(text: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)^[\.\#]\S*").unwrap();
    }
    RE.find_iter(text).map(|mat| mat.as_str()).collect()
}

fn find_declarations(text: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"readonly '\S*': \w*").unwrap();
    }
    RE.find_iter(text).map(|mat| mat.as_str()).collect()
}

fn remove_modifiers(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[\.\#]").unwrap();
    }
    let mut __san_name = Vec::new();
    if text.contains(':') {
        __san_name = text.split(':').collect();
        RE.replace_all(__san_name[0], "").to_string()
    } else {
        RE.replace_all(text, "").to_string()
    }
}

fn camel_case_converter(text: &str) -> String {
    let out: Vec<&str> = text.split('-').collect();
    let mut parsed_name = String::new();
    for (i, word) in out.into_iter().enumerate() {
        let mut __name_indiv = String::new();
        let (first_char, remainder) = split_first_char(word);
        if i == 0 {
            __name_indiv = first_char.to_lowercase() + &remainder.to_lowercase();
        } else {
            __name_indiv = first_char.to_uppercase() + &remainder.to_lowercase();
        }
        parsed_name = parsed_name + &__name_indiv;
    }
    parsed_name
}

fn kebab_case_converter(text: &str) -> String {
    // find lowcase/titlecase followed by uppercase/titlecase/numbers Or numbers followed by lowercase letter
    // replace with value of capture groups
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)([\p{Ll}\p{Lt}])([\p{Lu}\p{Lt}\p{Nd}\p{Nl}\p{No}]) | ([\p{Nd}\p{Nl}\p{No}])(\p{Ll})").unwrap();
    }
    RE.replace_all(text, "${1}${3}-${2}${4}")
        .to_string()
        .to_lowercase()
}

fn split_first_char(s: &str) -> (&str, &str) {
    for i in 1..5 {
        let r = s.get(0..i);
        if let Some(x) = r {
            return (x, &s[i..]);
        }
    }
    (&s[0..0], s)
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
        let declarations =
            find_declarations("readonly 'test': string;\n readonly 'test2': string;");
        let declarations_expected = ["readonly 'test': string", "readonly 'test2': string"];
        assert_eq!(declarations, declarations_expected)
    }

    #[test]
    fn parse_and_print_files() {
        let paths_expected = (
            "./test/test.module.css",
            "./test/recursive_test/test_r.module.css",
        );
        parse_and_print(
            &[paths_expected.0.to_string(), paths_expected.1.to_string()],
            ModFlags {
                camel_case_flag: false,
                kebab_case_flag: false,
                out_dir: &String::new(),
            },
        );
        let path_exists = (
            Path::new(paths_expected.0).exists(),
            Path::new(paths_expected.1).exists(),
        );
        assert_eq!(path_exists, (true, true))
    }

    #[test]
    fn split_string() {
        let (first_char, remainder) = split_first_char("test");
        assert_eq!(first_char, "t");
        assert_eq!(remainder, "est")
    }

    #[test]
    fn remove_mods() {
        let parsed_name = remove_modifiers("#test:modifiers");
        assert_eq!(parsed_name, "test")
    }

    #[test]
    fn camel_case() {
        let name = camel_case_converter("Hello-world");
        assert_eq!(name, "helloWorld");
    }

    #[test]
    fn kebab_case() {
        let name = kebab_case_converter("helloWorldTest");
        assert_eq!(name, "hello-world-test");
    }

    #[test]
    fn get_f_data() {
        let file_data_expected: (Vec<String>, String) = (
            vec![
                "readonly 'test-class': string;".to_string(),
                "readonly 'test-Id': string;".to_string(),
            ],
            "./test/test.module.css.d.ts".to_string(),
        );
        let file_data = get_file_data(
            &"./test/test.module.css".to_string(),
            &ModFlags {
                camel_case_flag: false,
                kebab_case_flag: false,
                out_dir: &String::new(),
            },
        );
        assert_eq!(file_data, file_data_expected)
    }

    #[test]
    fn get_f_r_data() {
        let file_data_expected: (Vec<String>, String) = (
            vec![
                "readonly 'R-test-Class': string;".to_string(),
                "readonly 'RtestId': string;".to_string(),
            ],
            "./test/recursive_test/test_r.module.css.d.ts".to_string(),
        );
        let file_data = get_file_data(
            &"./test/recursive_test/test_r.module.css".to_string(),
            &ModFlags {
                camel_case_flag: false,
                kebab_case_flag: false,
                out_dir: &String::new(),
            },
        );
        assert_eq!(file_data, file_data_expected)
    }
}

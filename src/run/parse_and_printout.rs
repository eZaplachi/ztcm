use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, path};

// Logic function
pub fn parse_and_print(path_names: &[String], camel_case_flag: bool) {
    for path in path_names {
        let (data_vec, outfile_name) = get_file_data(path, camel_case_flag);
        print_files(data_vec, outfile_name);
    }
}

// Helper logic functions
fn get_file_data(path: &String, camel_case_flag: bool) -> (Vec<String>, String) {
    let outfile_path = format!("{}.d.ts", path);
    let contents = fs::read_to_string(path).expect("Something went wrong reading the .css file");
    let mut out_names = Vec::new();
    let re = Regex::new(r"[\.\#]").unwrap();
    let mut __out_name = String::new();
    let names = find_classes_or_ids(&contents);
    for name in names {
        if camel_case_flag {
            let camel_name = camel_case_converter(name);
            __out_name = format!("readonly '{}': string;", camel_name);
            out_names.push(__out_name)
        } else {
            let parsed_name = re.replace_all(name, "");
            __out_name = format!("readonly '{}': string;", parsed_name);
            out_names.push(__out_name)
        }
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

// Regex functions
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

fn split_first_char(s: &str) -> (&str, &str) {
    for i in 1..5 {
        let r = s.get(0..i);
        if let Some(x) = r {
            return (x, &s[i..]);
        }
    }
    (&s[0..0], s)
}

fn format_fist_letter(first_char: &str, remainder: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\w").unwrap();
    }
    if RE.is_match(first_char) {
        let formatted_first_char_upper = &first_char.to_uppercase();
        formatted_first_char_upper.to_string() + remainder
    } else {
        let (intermediate_first_char, intermediate_remainder) = split_first_char(remainder);
        let formatted_first_char_lower = &intermediate_first_char.to_lowercase();
        formatted_first_char_lower.to_string() + intermediate_remainder
    }
}

fn remove_modifiers(text: &str) -> &str {
    let mut __parsed_name = Vec::new();
    if text.contains(':') {
        __parsed_name = text.split(':').collect();
        __parsed_name[0]
    } else {
        text
    }
}

fn camel_case_converter(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-").unwrap();
    }
    let out: Vec<&str> = RE.split(text).collect();
    let mut names: Vec<String> = Vec::new();
    for word in out {
        let parsed_name = remove_modifiers(word);
        let (first_char, remainder) = split_first_char(parsed_name);
        let name_indiv: String = format_fist_letter(first_char, remainder);
        names.push(name_indiv);
    }
    let mut parsed_name = String::new();
    for name in names {
        parsed_name = parsed_name + &name
    }
    parsed_name
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
            false,
        );
        let path_exists = (
            path::Path::new(paths_expected.0).exists(),
            path::Path::new(paths_expected.1).exists(),
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
        let parsed_name = remove_modifiers("test:modifiers");
        assert_eq!(parsed_name, "test")
    }

    #[test]
    fn format_cc() {
        let first_word = format_fist_letter(".", "Test");
        let subsequent_word = format_fist_letter("t", "est");
        assert_eq!(first_word, "test");
        assert_eq!(subsequent_word, "Test")
    }

    #[test]
    fn camel_case() {
        let name = camel_case_converter(".Hello-world:modifiers");
        assert_eq!(name, "helloWorld");
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
        let file_data = get_file_data(&"./test/test.module.css".to_string(), false);
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
            false,
        );
        assert_eq!(file_data, file_data_expected)
    }
}

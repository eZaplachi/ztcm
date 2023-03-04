use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashSet,
    fs::{self, create_dir_all},
    path::Path,
};

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

fn get_file_data(path: &String, mod_flags: &ModFlags) -> (HashSet<String>, String) {
    let mut __outfile_name = String::new();
    if !mod_flags.out_dir.is_empty() {
        if !Path::new(mod_flags.out_dir).exists() {
            create_dir_all(mod_flags.out_dir).expect("Couldn't create output directory");
        }
        let mod_path = mod_flags.out_dir.to_owned()
            + "/"
            + path.split('/').last().expect("Error parsing file name");
        __outfile_name = format!("{}.d.ts", mod_path);
    } else {
        __outfile_name = format!("{}.d.ts", path);
    }
    let contents = fs::read_to_string(path).expect("Something went wrong reading the .css file");
    let mut out_names = HashSet::new();
    let mut __out_name = String::new();
    let names = find_classes_or_ids(&contents);
    for name in names {
        let split_names = split_classes_ids(name);
        for split_name in split_names {
            let parsed_name = remove_modifiers(name);
            if !check_reserved(parsed_name.to_string()) {
                if mod_flags.camel_case_flag {
                    let camel_name = camel_case_converter(&parsed_name);
                    __out_name = format_line(camel_name);
                    out_names.insert(__out_name);
                } else if mod_flags.kebab_case_flag {
                    let kebab_name = kebab_case_converter(&parsed_name);
                    __out_name = format_line(kebab_name);
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

fn print_files(data_set: HashSet<String>, outfile_name: String) {
    let path_exists = Path::new(&outfile_name).exists();
    let mut _outfile_data = String::new();
    let mut outfile_set = HashSet::new();
    let mut matching_value = false;
    let mut print_out = false;
    if !path_exists {
        println!("Creating file: {}", outfile_name);
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
        println!("\rWrote to file: {}", outfile_name)
    }
}

fn find_classes_or_ids(text: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)^[\.\#]+[^\{]*|^@value\s\S*").unwrap();
    }
    RE.find_iter(text).map(|mat| mat.as_str().trim()).collect()
}

fn find_declarations(text: &str) -> HashSet<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"readonly '\S*': \w*").unwrap();
    }
    RE.find_iter(text).map(|mat| mat.as_str()).collect()
}

fn remove_modifiers(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[\.\#]|@value\s").unwrap();
    }
    let mut __san_name = Vec::new();
    if text.contains(':') {
        __san_name = text.split(':').collect();
        RE.replace_all(__san_name[0], "").to_string()
    } else {
        RE.replace_all(text, "").to_string()
    }
}

fn check_banned(name: String) {}

fn check_reserved(word: String) -> bool {
    let res_string = fs::read_to_string("src/run/reserved_words_ts.txt")
        .expect("Error - Couldn't read reserved words file");
    let res_vec = res_string.split("\n");
    for res in res_vec {
        if word == res {
            return true;
        }
    }
    false
}

// Used to avoid bad imports ex. .test1, #test2, {}
fn split_classes_ids(name: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r",+?").unwrap();
    }
    let parsed_name: Vec<&str> = RE.split(&name).collect();
    let mut out_name = Vec::new();
    if parsed_name.len() > 1 {
        for indiv_name in parsed_name {
            out_name.push(indiv_name.trim());
        }
    } else {
        out_name.push(name);
    }
    return out_name;
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
        static ref RE: Regex = Regex::new(
            r"(?x)([\p{Ll}\p{Lt}])([\p{Lu}\p{Lt}\p{Nd}\p{Nl}\p{No}])|([\p{Nd}\p{Nl}\p{No}])(\p{Ll})"
        )
        .unwrap();
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
        let class_or_id = find_classes_or_ids(".testClass {}\n#testId {}\n@value test");
        let class_or_id_expected = [".testClass", "#testId", "@value test"];
        assert_eq!(class_or_id, class_or_id_expected)
    }

    #[test]
    fn find_decls() {
        let declarations =
            find_declarations("readonly 'test': string;\n readonly 'test2': string;");
        let declarations_expected = HashSet::from(["readonly 'test': string", "readonly 'test2': string"]);
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
                kebab_case_flag: false,
                out_dir: &String::new(),
            },
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
                kebab_case_flag: false,
                out_dir: &"./test/test_outdir".to_string(),
            },
        );
        assert_eq!(Path::new(paths_expected[1]).exists(), true)
    }

    #[test]
    fn split_string() {
        let (first_char, remainder) = split_first_char("test");
        assert_eq!(first_char, "t");
        assert_eq!(remainder, "est")
    }

    #[test]
    fn remove_mods() {
        let parsed_id = remove_modifiers("#test:modifiers");
        let parsed_val = remove_modifiers("@value test");
        assert_eq!(
            (parsed_id, parsed_val),
            ("test".to_string(), "test".to_string())
        )
    }

    #[test]
    fn check_res() {
        let res_true = check_reserved("any".to_string());
        let res_false = check_reserved("test".to_string());
        assert_eq!((res_true, res_false), (true, false))
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
        let file_data_expected: (HashSet<String>, String) = (
            HashSet::from([
                "readonly 'test-class': string;".to_string(),
                "readonly 'test-Id': string;".to_string(),
            ]),
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
        let file_data_expected: (HashSet<String>, String) = (
            HashSet::from([
                "readonly 'R-test-Class': string;".to_string(),
                "readonly 'RtestId': string;".to_string(),
            ]),
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

use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashSet,
    fs::{self, create_dir_all},
    path::Path,
};

mod text;

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
        __outfile_name = format!("{}.d.ts", mod_path);
    } else {
        __outfile_name = format!("{}.d.ts", path);
    }
    let contents = fs::read_to_string(path).expect("Something went wrong reading the .css file");
    let san_contents = remove_comments(&contents);
    let mut out_names = HashSet::new();
    let mut __out_name = String::new();
    let names = get_classes_or_ids(&san_contents);
    for name in names {
        let split_names = split_classes_ids(name);
        for split_name in split_names {
            let parsed_name = remove_modifiers(split_name);
            if !check_reserved(parsed_name.to_string()) {
                if mod_flags.camel_case_flag {
                    let camel_name = camel_case_converter(&parsed_name);
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

fn get_classes_or_ids(text: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[\.\#]+(.+?)\{+|^@value\s\S*").unwrap();
    }
    let found_cid = find_classes_or_ids(text);
    let mut valid_cid = vec![];
    for cid in found_cid {
        if RE.is_match(cid) {
            valid_cid.push(RE.find(cid).unwrap().as_str().remove_last().trim());
        }
    }
    valid_cid
}

fn find_classes_or_ids(text: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?ms)^[\.\#]{1}(.+?)\{{1}(.*?)\}{1}|^@value+(.+?);").unwrap();
    }
    RE.find_iter(text).map(|mat| mat.as_str()).collect()
}

// Move below to different file
trait StrExt {
    fn remove_last(&self) -> &str;
}

impl StrExt for str {
    fn remove_last(&self) -> &str {
        match self.char_indices().next_back() {
            Some((i, _)) => &self[..i],
            None => self,
        }
    }
}

fn remove_comments(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?s)/\*(.*?)\*/").unwrap();
    }
    RE.replace_all(text, "").into_owned()
}

fn remove_modifiers(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[\.\#]|@value\s|\s|;").unwrap();
    }
    let mut __san_name = Vec::new();
    if text.contains(':') {
        __san_name = text.split(':').collect();
        RE.replace_all(__san_name[0], "").to_string()
    } else {
        RE.replace_all(text, "").to_string()
    }
}

fn check_reserved(word: String) -> bool {
    let res_vec = text::ts_reserved_words();
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
    let parsed_name: Vec<&str> = RE.split(name).collect();
    let mut out_name = Vec::new();
    if parsed_name.len() > 1 {
        for indiv_name in parsed_name {
            out_name.push(indiv_name.trim());
        }
    } else {
        out_name.push(name);
    }
    out_name
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
    fn test_get_classes_or_ids() {
        let class_or_id =
            get_classes_or_ids(".testClass {}\n#testId {}\n.errorClass {\n@value test;");
        let class_or_id_expected = [".testClass", "#testId", "@value test"];
        assert_eq!(class_or_id, class_or_id_expected)
    }

    #[test]
    fn split_string() {
        let (first_char, remainder) = split_first_char("test");
        assert_eq!(first_char, "t");
        assert_eq!(remainder, "est")
    }

    #[test]
    fn test_remove_comments() {
        let text = "/*\n.commentClass {}\n*/\n.test{}".to_string();
        let parsed_text = remove_comments(&text);
        assert_eq!(parsed_text, "\n.test{}")
    }

    #[test]
    fn test_remove_modifiers() {
        let parsed_id = remove_modifiers("#test:modifiers");
        let parsed_val = remove_modifiers("@value test");
        assert_eq!(
            (parsed_id, parsed_val),
            ("test".to_string(), "test".to_string())
        )
    }

    #[test]
    fn check_reserved_names() {
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
    fn test_parse_file_data() {
        let file_data_expected: (HashSet<String>, String) = (
            HashSet::from([
                "readonly 'test-class': string;".to_string(),
                "readonly 'test-id': string;".to_string(),
                "readonly 'test': string;".to_string(),
                "readonly 'split-test': string;".to_string(),
            ]),
            "./test/test.module.css.d.ts".to_string(),
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

    #[test]
    fn test_get_file_recursive_data() {
        let file_data_expected: (HashSet<String>, String) = (
            HashSet::from([
                "readonly 'R-test-Class': string;".to_string(),
                "readonly 'RtestId': string;".to_string(),
            ]),
            "./test/recursive_test/test_r.module.css.d.ts".to_string(),
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

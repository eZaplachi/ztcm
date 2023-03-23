use lazy_static::lazy_static;
use regex::Regex;

pub trait StrExt {
    fn remove_last(&self) -> &str;
    fn remove_comments(&self) -> String;
    fn remove_modifiers(&self) -> String;
    fn get_classes_or_ids(&self) -> Vec<&str>;
    fn find_classes_or_ids(&self) -> Vec<&str>;
    fn split_classes_ids(&self) -> Vec<&str>;
    fn camel_case_converter(&self) -> String;
    fn split_first_char(&self) -> (&str, &str);
}

impl StrExt for str {
    fn remove_last(&self) -> &str {
        match self.char_indices().next_back() {
            Some((i, _)) => &self[..i],
            None => self,
        }
    }
    fn remove_comments(&self) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?s)/\*(.*?)\*/").unwrap();
        }
        RE.replace_all(self, "").to_string()
    }
    fn remove_modifiers(&self) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[\.\#]|@value\s|\s|;").unwrap();
        }
        let mut __san_name = Vec::new();
        if self.contains(':') {
            __san_name = self.split(':').collect();
            RE.replace_all(__san_name[0], "").to_string()
        } else {
            RE.replace_all(self, "").to_string()
        }
    }
    fn get_classes_or_ids(&self) -> Vec<&str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[\.\#]+(.+?)\{+|^@value\s\S*").unwrap();
        }
        let found_cid = self.find_classes_or_ids();
        let mut valid_cid = vec![];
        for cid in found_cid {
            if RE.is_match(cid) {
                valid_cid.push(RE.find(cid).unwrap().as_str().remove_last().trim());
            }
        }
        valid_cid
    }

    fn find_classes_or_ids(&self) -> Vec<&str> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?ms)^[\.\#]{1}(.+?)\{{1}(.*?)\}{1}|^@value+(.+?);").unwrap();
        }
        RE.find_iter(self).map(|mat| mat.as_str()).collect()
    }
    // Used to avoid bad imports ex. .test1, #test2, {}
    fn split_classes_ids(&self) -> Vec<&str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r",+?").unwrap();
        }
        let parsed_name: Vec<&str> = RE.split(self).collect();
        let mut out_name = Vec::new();
        if parsed_name.len() > 1 {
            for indiv_name in parsed_name {
                out_name.push(indiv_name.trim());
            }
        } else {
            out_name.push(self);
        }
        out_name
    }

    fn camel_case_converter(&self) -> String {
        let out: Vec<&str> = self.split('-').collect();
        let mut parsed_name = String::new();
        for (i, word) in out.into_iter().enumerate() {
            let mut __name_indiv = String::new();
            let (first_char, remainder) = word.split_first_char();
            if i == 0 {
                __name_indiv = first_char.to_lowercase() + &remainder.to_lowercase();
            } else {
                __name_indiv = first_char.to_uppercase() + &remainder.to_lowercase();
            }
            parsed_name = parsed_name + &__name_indiv;
        }
        parsed_name
    }

    fn split_first_char(&self) -> (&str, &str) {
        for i in 1..5 {
            let r = self.get(0..i);
            if let Some(x) = r {
                return (x, &self[i..]);
            }
        }
        (&self[0..0], self)
    }
}

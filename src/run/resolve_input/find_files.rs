use glob::glob;
use std::{fs, io};

pub fn get_files(directory: String) -> io::Result<Vec<String>> {
    let mut entries = fs::read_dir(directory)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    // The entries have now been sorted by their path.
    let mut css_files_path: Vec<String> = Vec::new();

    for entry in entries {
        let entry_string: String = entry.as_path().display().to_string();
        let files = &entry_string.ends_with(".css");
        if files == &true {
            css_files_path.push(entry_string.clone())
        }
    }
    Ok(css_files_path)
}

pub fn get_files_recursive(directory: String) -> Vec<String> {
    let chosen_folder = directory + "/**/*.css";

    let mut css_file_paths: Vec<String> = Vec::new();
    for entry in glob(chosen_folder.as_str()).expect("Failed to read file names recursively") {
        if let Ok(path) = entry {
            css_file_paths.push(path.display().to_string());
        } else if let Err(e) = entry {
            println!("Glob error: {:?}", e)
        }
    }
    css_file_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_file() {
        let mut test_output_found = false;
        let test_output_expected = "./test/test.module.css";

        let files = get_files("./test".to_string()).unwrap();
        for file in files {
            if file == test_output_expected {
                test_output_found = true;
            }
        }

        assert_eq!(test_output_found, true)
    }

    #[test]
    fn get_file_r() {
        let mut test_output_found = (false, false);
        let test_output_expected = (
            "test/test.module.css",
            "test/recursive_test/test_r.module.css",
        );

        let files_r = get_files_recursive("./test".to_string());
        for file in files_r {
            if file == test_output_expected.0 {
                test_output_found.0 = true;
            }
            if file == test_output_expected.1 {
                test_output_found.1 = true;
            }
        }
        assert_eq!(test_output_found, (true, true))
    }
}

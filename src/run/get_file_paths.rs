use glob::glob;

pub fn get_files(directory: String, pattern: String) -> Vec<String> {
    let chosen_folder = directory + "/*" + &pattern;

    let mut css_file_paths: Vec<String> = Vec::new();
    for entry in glob(chosen_folder.as_str()).expect("Failed to read file names") {
        if let Ok(path) = entry {
            css_file_paths.push(path.display().to_string());
        } else if let Err(e) = entry {
            println!("Glob error: {:?}", e)
        }
    }
    css_file_paths
}

pub fn get_files_recursive(directory: String, pattern: String) -> Vec<String> {
    let chosen_folder = directory + "/**/*" + &pattern;

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
        let test_output_expected = "test/test.module.css";

        let files = get_files("./test".to_string(), ".module.css".to_string());
        for file in files {
            if file == test_output_expected {
                test_output_found = true;
            }
        }

        assert_eq!(test_output_found, true)
    }

    #[test]
    fn get_file_recursive() {
        let mut test_output_found = (false, false);
        let test_output_expected = (
            "test/test.module.css",
            "test/recursive_test/test_r.module.css",
        );

        let files_r = get_files_recursive("./test".to_string(), ".module.css".to_string());
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

    #[test]
    fn get_file_path() {
        let mut test_output_found = false;
        let test_output_expected = "test/test.module.scss";

        let files = get_files("./test".to_string(), ".scss".to_string());
        for file in files {
            if file == test_output_expected {
                test_output_found = true;
            }
        }

        assert_eq!(test_output_found, true)
    }
}

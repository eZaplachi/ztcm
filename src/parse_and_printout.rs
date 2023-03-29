use std::{thread, time::Instant};
pub mod parse;
mod print;
mod str_ext;
use parse::{parse_file_data, ModFlags};

pub fn parse_and_printout(
    paths: &Vec<String>,
    threads: i32,
    camel_case_flag: bool,
    out_dir: &String,
    timer: bool,
) {
    let mut _is_multithreaded = false;
    if threads > 1 {
        _is_multithreaded = true;
    }
    let num_of_paths = paths.len();
    if threads as usize > num_of_paths {
        panic!(
            "Error - More threads ({}) than files ({})",
            threads, num_of_paths
        )
    }
    let files_per_thread = (num_of_paths as f32 / threads as f32).ceil();
    thread::scope(|s| {
        for i in 0..threads {
            let start_index: f32 = i as f32 * files_per_thread;
            let mut end_index: f32 = (i + 1) as f32 * files_per_thread;
            if end_index > num_of_paths as f32 {
                end_index = num_of_paths as f32;
            }
            let paths_part: Vec<_> = paths[start_index as usize..end_index as usize].to_vec();
            let handle = s.spawn(move || {
                parse_and_print(
                    &paths_part,
                    ModFlags {
                        camel_case_flag,
                        out_dir,
                    },
                    _is_multithreaded,
                    i + 1,
                    timer,
                );
            });
            handle.join().unwrap();
        }
    });
}

fn parse_and_print(
    path_names: &[String],
    mod_flags: ModFlags,
    is_multithreaded: bool,
    thread_num: i32,
    timer: bool,
) {
    for path in path_names {
        if !path.contains("global") {
            let now_parse = Instant::now();
            let (data_vec, outfile_name) = parse_file_data(path, &mod_flags);
            let elapsed_time_parse = now_parse.elapsed();
            let parsed_time_text = format!(
                "Parsed {} in {} microseconds",
                path,
                elapsed_time_parse.as_micros(),
            );
            if timer {
                if is_multithreaded {
                    print_multithreaded(parsed_time_text, thread_num);
                } else {
                    println!("{}", parsed_time_text);
                }
            }
            if !data_vec.is_empty() {
                let now_print = Instant::now();
                print::print_files(
                    data_vec,
                    outfile_name.clone(),
                    is_multithreaded,
                    thread_num.clone(),
                );
                let elapsed_time_print = now_print.elapsed();
                let printed_time_text = format!(
                    "Compared .css data to {} in {} microseconds",
                    outfile_name,
                    elapsed_time_print.as_micros(),
                );
                if timer {
                    if is_multithreaded {
                        print_multithreaded(printed_time_text, thread_num);
                    } else {
                        println!("{}", printed_time_text);
                    }
                }
            }
        }
    }
}

pub fn print_multithreaded(text: String, thread_num: i32) {
    println!("{} (T{})", text, thread_num)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

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
            false,
            1,
            false,
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
            false,
            1,
            false,
        );
        assert_eq!(Path::new(paths_expected[1]).exists(), true)
    }

    #[should_panic]
    #[test]
    fn parse_and_print_too_many_threads() {
        parse_and_printout(
            &vec!["./test/test.module.css".to_string()],
            9999,
            false,
            &"./test/".to_string(),
            false,
        );
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
            false,
            1,
            false,
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
            false,
            1,
            false,
        );
        assert_eq!(Path::new(paths_expected[1]).exists(), true)
    }
}

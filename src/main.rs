use std::{env, process};
mod resolve_input;
mod parse_and_printout;
use resolve_input::Config;
use parse_and_printout::ParseRes;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let config_res = resolve_input::run(config).unwrap_or_else(|err| {
        println!("Problem parsing file paths: {}", err);
        process::exit(1);
    });

    ParseRes::parse_and_print_out(config_res.0, config_res.1);
}

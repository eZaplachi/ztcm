use std::env;
mod run;
use run::run_ztcm;

fn main() {
    let args: Vec<String> = env::args().collect();
    run_ztcm(&args)
}

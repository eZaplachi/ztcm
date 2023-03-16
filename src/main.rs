use clap::Parser;
mod run;
use run::args::Cli;
use run::run_ztcm;

fn main() {
    let cli: Cli = Cli::parse();
    // println!("{cli:#?}");
    let mut _files_paths: Vec<String> = vec![];
    // let args: Vec<String> = env::args().collect();
    run_ztcm(cli)
}

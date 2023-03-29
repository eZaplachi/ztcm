use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to .css files; '.' for current directory
    pub path: String,
    #[arg(short, long)]
    /// Converts output from kebab-case to camelCase in .d.ts files
    pub camel_case: bool,
    /// Enable multithreaded mode and set the number of threads to run with  -- Can't exceed number of .css files
    #[arg(short, long, value_name = "THREADS", value_parser = clap::value_parser!(i32).range(1..32), default_value="1", num_args=0..=1, require_equals=true, default_missing_value = "2")]
    pub multithread: i32,
    /// Will default to outputting .d.ts files in the same directory as .css files
    #[arg(short, long, value_name = "OUTPUT DIRECTORY", default_value = "", num_args=0..=1, require_equals = true)]
    pub output: String,
    /// Set a custom pattern for .css like files (eg. .icss)
    #[arg(short, long, default_value_t = String::from(".module.css"), require_equals=true)]
    pub pattern: String,
    /// Search the given PATH recursively
    #[arg(short, long)]
    pub recursive: bool,
    /// Shows the ammount of time it takes to run each step
    #[arg(short, long)]
    pub timer: bool,
    /// Set number of watch cycles to pass before re-indexing files
    #[arg(short, long, value_parser = clap::value_parser!(i32), default_value = "45", num_args=0..=1, require_equals=true)]
    pub update_after_cycles: i32,
    /// Enable watch and optionally set the watch delay
    #[arg(short, long, value_name="DELAY(s)", value_parser = clap::value_parser!(f64), default_value="0", num_args=0..=1, require_equals=true, default_missing_value = "1.0")]
    pub watch: f64,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}

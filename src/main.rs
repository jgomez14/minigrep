use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let app_config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {err}");

        process::exit(1);
    });

    if let Err(e) = minigrep::run(app_config) {
        println!("Application error: {e}");

        process::exit(1);
    };
}

use std::fs;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let app_config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {err}");

        process::exit(1);
    });

    println!("Searching for pattern '{}'", app_config.query);
    println!("\nIn file '{}'", app_config.file_path);

    let file_content = fs::read_to_string(app_config.file_path)
        .expect("ERROR reading file.");

    println!("With contents:\n{}", file_content)
}

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

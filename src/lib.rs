use std::fs;
use std::error::Error;

pub fn run(app_config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(app_config.file_path)?;

    println!("With contents:\n{}", file_content);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

use std::error::Error;
use std::fs;

use args_config::ArgsConfig;

mod args_config;
mod seeker;

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = ArgsConfig::build(std::env::args().collect())?;
    let content = fs::read_to_string(config.file_path())?;
    let result = seeker::search(config.query(), &content, config.ignore_case());
    println!("{}", result.join("\n"));
    Ok(())
}
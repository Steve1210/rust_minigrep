use std::env;
use std::fs;
use std::process;
use std::error::Error;

use minigrep::{search, search_case_insensitive};

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    let config = Config::build(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {e}");
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.file_path);

    // We don't use `unwrap_or_else` because run() has no return value to be unwrapped
    // so, we only check if the returned value is an error
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next(); // Skip the first argument, which is the program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {query, file_path, ignore_case})
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
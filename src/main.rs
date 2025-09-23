use minigrep::{search, search_case_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let (_lower, upper) = args.size_hint();

        if let Some(ub) = upper {
            if ub < 2 {
                return Err(
                    "not enough arguments: usage: <query> <file_path> [--ignore-case|--case-sensitive]",
                );
            }
        }

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let mut ignore_case = match env::var("IGNORE_CASE") {
            Ok(v) if matches!(v.to_lowercase().as_str(), "1" | "true") => true,
            _ => false,
        };

        if let Some(flag) = args.next() {
            match flag.as_str() {
                "--ignore-case" => ignore_case = true,
                "--case-sensitive" => ignore_case = false,
                _ => {
                    return Err("invalid argument: expected --ignore-case or --case-sensitive");
                }
            }
        }

        if args.next().is_some() {
            return Err("too many arguments");
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

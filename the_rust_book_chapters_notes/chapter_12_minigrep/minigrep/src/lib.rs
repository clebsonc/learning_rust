use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: &Config) -> Result <(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }
    else {
        search(&config.query, &contents)
    };

    for line in result.iter() {
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
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments.");
        }
        let query = args.get(1).unwrap();
        let file_path = args.get(2).unwrap();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query: query.clone(),
            file_path: file_path.clone(),
            ignore_case,
        })
    }
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut found: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            found.push(line);
        }
    }
    found
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut found: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            found.push(line);
        }
    }
    found
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "Duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["Duct tape."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

use std::env;
use std::error::Error;
use std::fs;

pub fn get_env_args() -> Vec<String> {
    env::args().collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = get_case_sensitive(&args);

        Ok(Config { query, filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
pub fn get_case_sensitive(args: &[String]) -> bool {
    let mut case_sensitive = true;
    if args.len() > 3 {
        if args[3] == "c" {
            case_sensitive = false;
        }

    }
    else {
            case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    }
    case_sensitive
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
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_case_insensitive_test() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], 
                   search_case_insensitive(query, contents));
    }

    #[test]
    fn test_if_case_insensitive() {
        let args = [String::from("test"),String::from("test"), String::from("c")];
        assert_eq!(get_case_sensitive(&args), false);
    }

    #[test]
    fn test_if_case_sensitive() {
        let args = [String::from("test"),String::from("test"), String::from("x")];
        assert_eq!(get_case_sensitive(&args), false);
    }
}

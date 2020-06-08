use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    query: String,
    file_name: String,
    case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("not enough arguments")
        } else {
            args.next();
            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string"),
            };
            let file_nmame = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file name"),
            };
            Ok(Config {
                query: query,
                file_name: file_nmame,
                case_insensitive: env::var("CASE_INSENSITIVE").is_ok(),
            })
        }
    }
}

/// Iterators are one of Rust’s zero-cost abstractions, by which
/// we mean using the abstraction imposes no additional runtime overhead.
/// https://doc.rust-lang.org/book/ch13-04-performance.html
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| line.trim())
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_case_insensitive = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query_case_insensitive))
        .map(|line| line.trim())
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;
    let rs = if config.case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    println!("Search Result:");
    for line in rs {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "Rust".to_string();
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three."
            .to_string();
        assert_eq!(vec!["Rust:"], search(&query, &contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rust".to_string();

        env::set_var("CASE_INSENSITIVE", "1");
        assert!(env::var("CASE_INSENSITIVE").is_ok());
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        TRust me."
            .to_string();
        assert_eq!(
            vec!["Rust:", "TRust me."],
            search_case_insensitive(&query, &contents)
        )
    }
}

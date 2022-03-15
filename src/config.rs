use clap::ArgMatches;
use colored::Colorize;
use regex::{Captures, RegexBuilder};

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub ignore_case: bool,
    pub count_lines_only: bool,
}

impl<'a> Config<'a> {
    pub fn new(matches: &'a ArgMatches) -> Result<Config<'a>, &'a str> {
        let query = matches.value_of("QUERY").expect("Missing query.");
        let filename = matches.value_of("FILENAME").expect("Missing filename.");
        let ignore_case = matches.is_present("ignore-case");
        let count_lines_only = matches.is_present("count");

        Ok(Config { query, filename, ignore_case, count_lines_only })
    }
}

pub fn search_case_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<String> {
    contents.lines()
        .filter(|l| l.contains(query))
        .map(|l| l.replace(query, &format!("{}", query.red())))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<String> {
    let rx = RegexBuilder::new(query)
        .case_insensitive(true)
        .build()
        .expect("Invalid Regex");

    contents.lines()
        .filter(|l| rx.captures(l).is_some())
        .map(|l| rx.replace(l,
                            |caps: &Captures| caps.iter().map(|m| {
                                &format!("{}", m.unwrap().as_str().red())
                            }),
        ))
        .collect()
}

pub fn count_lines<'a>(query: &'a str, contents: &'a str) -> i32 {
    contents.lines().fold(0, |acc, l| {
        if l.contains(query) {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

    #[test]
    fn test_count_lines() {
        let query = "rust";
        let contents = "\
rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(2, count_lines(query, contents));
    }
}

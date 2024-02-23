use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // We’re using the is_ok method on the Result to check whether the environment variable is set,
        // which means the program should do a case-insensitive search.
        // If the IGNORE_CASE environment variable isn’t set to anything,
        // is_ok will return false and the program will perform a case-sensitive search.
        // We don’t care about the value of the environment variable, just whether it’s set or unset,
        // so we’re checking is_ok rather than using unwrap, expect,
        // or any of the other methods we’ve seen on Result.
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
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

/// Search function:
/// Notice that we need to define an explicit lifetime 'a in the signature of search
/// and use that lifetime with the contents argument and the return value.
/// Recall in Chapter 10 that the lifetime parameters specify which argument lifetime
/// is connected to the lifetime of the return value. In this case, we indicate that
/// the returned vector should contain string slices that reference slices of
/// the argument contents (rather than the argument query).
/// In other words, we tell Rust that the data returned by the search function
/// will live as long as the data passed into the search function in the contents argument.
/// This is important!
/// The data referenced by a slice needs to be valid for the reference to be valid;
/// if the compiler assumes we’re making string slices of query rather than contents,
/// it will do its safety checking incorrectly.
///
/// our program needs to follow these steps:
/// 1. Iterate through each line of the contents.
/// 2. Check whether the line contains our query string.
/// 3. If it does, add it to the list of values we’re returning.
/// 4. If it doesn’t, do nothing.
/// 5. Return the list of results that match.
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
    // we lowercase the query string and store it in a shadowed variable with the same name
    // Note that query is now a String rather than a string slice,
    // because calling to_lowercase creates new data rather than referencing existing data
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "iDEATH";
        let contents = "\
iDEATH is a place where the sun shines
a different colour every day
and where people travel
to the length of their dreams.";

        assert_eq!(
            vec!["iDEATH is a place where the sun shines"],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "CoLoUr";
        let contents = "\
iDEATH is a place where the sun shines
a different colour every day
and where people travel
to the length of their dreams.";

        assert_eq!(
            vec!["a different colour every day"],
            search_case_insensitive(query, contents)
        );
    }
}

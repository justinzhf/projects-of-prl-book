use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        
        let query = match args.next() {
            Some(v) => v,
            None => return Err("not enough arguments."),
        };
        let filename = match args.next() {
            Some(v) => v,
            None => return Err("not enough arguments."),
        };
        
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
   let contents = fs::read_to_string(&config.filename)?;

   let results = if config.case_sensitive {
       search_case_sensitive(&config.query, &contents)
   } else {
       search_case_insensitive(&config.query, &contents)
   };

   for line in results {
       println!("{}", line);
   }
   Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    return results;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
    }

   #[test]
   fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            search_case_insensitive(query, contents)
        );
   }

}

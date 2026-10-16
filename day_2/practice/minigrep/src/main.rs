use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, String> {
        if args.len() != 2 {
            return Err("expected 2 arguments: <query> <file>".to_string());
        }

        let query = args[0].clone();
        let file_path = args[1].clone();

        let ignore_case = env::var("IGNORE_CASE")
            .map(|value| value == "1")
            .unwrap_or(false);

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

fn run(config: Config) -> Result<(), String> {
    let contents = fs::read_to_string(&config.file_path)
        .map_err(|_| format!("cannot read file '{}'", config.file_path))?;

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

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let config = Config::build(&args).unwrap_or_else(|error| {
        eprintln!("error: {error}");
        process::exit(1);
    });

    if let Err(error) = run(config) {
        eprintln!("error: {error}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let result = search("frog", "How public, like a frog\nHello");
        assert_eq!(result, vec!["How public, like a frog"]);
    }

    #[test]
    fn multiple_results() {
        let result = search("body", "I'm nobody!\nbody language\nGoodbye");
        assert_eq!(result, vec!["I'm nobody!", "body language"]);
    }

    #[test]
    fn no_results() {
        let result = search("cat", "frog\nhello");
        assert_eq!(result, Vec::<&str>::new());
    }

    #[test]
    fn case_insensitive_search() {
        let result = search_case_insensitive(
            "BODY",
            "I'm nobody! Who are you?\nAre you nobody, too?\nHello",
        );

        assert_eq!(
            result,
            vec!["I'm nobody! Who are you?", "Are you nobody, too?"]
        );
    }

    #[test]
    fn config_build_success() {
        let args = vec!["frog".to_string(), "poem.txt".to_string()];

        let config = Config::build(&args).unwrap();

        assert_eq!(config.query, "frog");
        assert_eq!(config.file_path, "poem.txt");
    }

    #[test]
    fn config_build_wrong_number_of_arguments() {
        let args = vec!["frog".to_string()];

        assert!(Config::build(&args).is_err());
    }
}

use std::env;
use std::error::Error;
use std::fs;


pub struct Config {
  query: String,
  file_name: String,
  case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Self, &str> {
    if args.len() < 3 {
      return Err("Something is wrong with arguments");
    }
    let query = args[1].to_string();
    let file_name = args[2].to_string();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    Ok(Self {
      query,
      file_name,
      case_sensitive,
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(&config.file_name)?;

  if config.case_sensitive {
    dbg!(search(&config.query, &contents))
  } else {
    dbg!(search_case_insensitive(&config.query, &contents))
  };
  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      result.push(line.trim());
    }
  }
  // println!("{:?}", result);
  result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  for line in contents.lines() {
    // let line_lowercase = line.to_lowercase();
    if line.to_lowercase().contains(&query.to_lowercase()) {
      result.push(line.trim());
    }
  }
  // println!("{:?}", result);
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "bla";
    let contents = "
    some result
    some result with bla
    other results
    ";

    assert_eq!(vec!["some result with bla"], search(query, contents));
  }

  #[test]
  fn multi_result() {
    let query = "result";
    let contents = "
    some result
    some result with bla
    other results
    ";

    assert_eq!(
      vec!["some result", "some result with bla", "other results"],
      search(query, contents)
    );
  }

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "
    Rust:
    safe, fast, productive.
    Pick three.
    Duct tape.
    ";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "duct";
    let contents = "
    Rust:
    safe, fast, productive.
    Pick three.
    Duct tape.
    ";
    assert_eq!(
      vec!["safe, fast, productive.", "Duct tape."],
      search_case_insensitive(query, contents)
    );
  }

}

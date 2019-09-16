
use std::error::Error;
use std::fs;

pub struct Config {
  query: String, // why pub?
  file_name: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Self, &str> {
    if args.len() < 3 {
      return Err("Something is wrong with arguments");
    }
    let query = args[1].to_string();
    let file_name = args[2].to_string();
    Ok(Self { query, file_name })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(&config.file_name)?;
  // println!("With text:\n{}", content);
  // for line in search(&config.query, &contents) {
  //   println!("Result: {}", line)
  // }

  let result = search(&config.query, &contents);
  println!("{:?}", result);
  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  // vec![]
  let mut result = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      result.push(line.trim());
    }
  }
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

    assert_eq!(vec!["some result with bla"], search(query, contents))
  }
}

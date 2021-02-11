pub struct Config {
  pub filename: String,
  pub query: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file name"),
    };

    let case_sensitive = std::env::var("CASE_INSENSITIVE=1").is_err();

    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
  let contents = std::fs::read_to_string(config.filename)?;

  let search_function = if config.case_sensitive {
    search
  } else {
    search_insensitive
  };

  search_function(&config.query, &contents)
    .iter()
    .for_each(|result| println!("{}", result));

  Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();

  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sensitive_search_zero_results() {
    let query = "vine";
    let contents = "\nRust:\nsafe, fast, productive.\nPick three.";

    let expected: Vec<String> = vec![];
    let result = search(query, contents);
    assert_eq!(expected, result);
  }

  #[test]
  fn sensitive_search_one_result() {
    let query = "duct";
    let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

    let expected = vec!["safe, fast, productive."];
    let result = search(query, contents);
    assert_eq!(expected, result);
  }

  #[test]
  fn sensitive_search_two_results() {
    let query = ".";
    let contents = "\nRust:\nsafe, fast, productive.\nPick three.";

    let expected = vec!["safe, fast, productive.", "Pick three."];
    let result = search(query, contents);
    assert_eq!(expected, result);
  }

  #[test]
  fn insensitive_search_two_results() {
    let query = "rUsT";
    let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nTrust me.";

    let expected = vec!["Rust:", "Trust me."];
    let result = search_insensitive(query, contents);
    assert_eq!(expected, result);
  }
}

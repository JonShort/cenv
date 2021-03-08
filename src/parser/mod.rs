use crate::utils::{Config, EnvContents, ParseStatus};
use regex::Regex;

lazy_static! {
  static ref KEYWORD_REGEX: Regex = Regex::new(r"^#+ *\+\+ *(\w+)").unwrap();
}

pub fn parse_env(env: EnvContents, config: &Config) -> EnvContents {
  let lines = env.contents.lines();

  let mut parse_status = ParseStatus::Ignore;

  let collected = lines.map(|line| {
    if is_empty(line) {
      parse_status = ParseStatus::Ignore;
      return String::from(line);
    }

    match resolve_keyword(line) {
      Some(kw) => {
        if kw == config.keyword {
          parse_status = ParseStatus::Active;
          return String::from(line);
        } else {
          parse_status = ParseStatus::Inactive;
          return String::from(line);
        }
      }
      None => (),
    };

    match parse_status {
      ParseStatus::Active => parse_as_active(line),
      ParseStatus::Inactive => parse_as_inactive(line),
      ParseStatus::Ignore => String::from(line),
    }
  });

  let collected: Vec<String> = collected.collect();
  let collected = collected.join("\n");

  EnvContents::new(collected)
}

fn is_empty(line: &str) -> bool {
  line == ""
}

fn parse_as_active(line: &str) -> String {
  let mut line_chars = line.chars();
  let result = match line_chars.next() {
    Some('#') => {
      line_chars.next();
      String::from(line_chars.as_str())
    }
    Some(_) => String::from(line),
    None => String::from(line),
  };

  result
}

fn parse_as_inactive(line: &str) -> String {
  let mut line_chars = line.chars();
  let result = match line_chars.next() {
    Some('#') => String::from(line),
    Some(_) => format!("{}{}", "# ", line),
    None => String::from(line),
  };

  result
}

fn resolve_keyword<'a>(line: &'a str) -> Option<&'a str> {
  let keyword = match KEYWORD_REGEX.captures(line) {
    Some(caps) => caps.get(1).map_or("", |m| m.as_str()),
    None => return None,
  };

  Some(&keyword)
}

#[cfg(test)]
mod is_empty_tests {
  use super::*;

  #[test]
  fn true_if_empty() {
    assert_eq!(is_empty(""), true);
  }

  #[test]
  fn false_if_not() {
    assert_eq!(is_empty(" "), false);
  }
}

#[cfg(test)]
mod parse_as_active_tests {
  use super::*;

  #[test]
  fn same_line_if_no_comment() {
    assert_eq!(parse_as_active("testing"), "testing");
  }

  #[test]
  fn same_line_if_empty() {
    assert_eq!(parse_as_active(""), "");
  }

  #[test]
  fn removes_hash() {
    assert_eq!(parse_as_active("# testing"), "testing");
  }
}

#[cfg(test)]
mod parse_as_inactive_tests {
  use super::*;

  #[test]
  fn same_line_if_comment() {
    assert_eq!(parse_as_inactive("#testing"), String::from("#testing"));
  }

  #[test]
  fn same_line_if_empty() {
    assert_eq!(parse_as_inactive(""), String::from(""));
  }

  #[test]
  fn adds_hash() {
    assert_eq!(parse_as_inactive("testing"), String::from("# testing"));
  }
}

#[cfg(test)]
mod resolve_keyword_tests {
  use super::*;

  #[test]
  fn none_if_not_formatted() {
    assert_eq!(resolve_keyword("SOME=thing"), None);
  }

  #[test]
  fn none_if_no_word() {
    assert_eq!(resolve_keyword("# ++ ++"), None);
  }

  #[test]
  fn none_if_no_comment() {
    assert_eq!(resolve_keyword("++ keyword ++"), None);
  }

  #[test]
  fn word_if_formatted_variant_1() {
    assert_eq!(resolve_keyword("# ++ keyword ++"), Some("keyword"));
  }

  #[test]
  fn word_if_formatted_variant_2() {
    assert_eq!(resolve_keyword("#++ keyword"), Some("keyword"));
  }

  #[test]
  fn word_if_formatted_variant_3() {
    assert_eq!(resolve_keyword("## ++ keyword ++"), Some("keyword"));
  }
}

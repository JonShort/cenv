//! Functions containing the core business logic around parsing.
//!
//! All of these functions encapsulate the "what" of cenv,
//! including:
//!
//! - What constitutes a "keyword"
//! - Which lines within the .env file should be updated
//! - The format that should be returned to the callee

use crate::utils::{Config, EnvContents};
use regex::Regex;

lazy_static! {
    static ref KEYWORD_REGEX: Regex = Regex::new(r"^#+ *\+\+ *(\w+)").unwrap();
}

#[derive(PartialEq)]
enum ParseStatus {
    Active,
    Inactive,
    Ignore,
}

fn parse_as_active(line: &str) -> String {
    let mut line_chars = line.chars();
    match line_chars.next() {
        Some('#') => {
            line_chars.next();
            String::from(line_chars.as_str())
        }
        Some(_) => String::from(line),
        None => String::from(line),
    }
}

fn parse_as_inactive(line: &str) -> String {
    let mut line_chars = line.chars();
    match line_chars.next() {
        Some('#') => String::from(line),
        Some(_) => format!("{}{}", "# ", line),
        None => String::from(line),
    }
}

/// Supplementary function which returns the keyword within the provided
/// line (if it exists)
///
/// This function accepts the EnvContents struct available in the
/// [utils](../utils/index.html) module.
pub fn resolve_keyword(line: &str) -> Option<&str> {
    let keyword = match KEYWORD_REGEX.captures(line) {
        Some(caps) => caps.get(1).map_or("", |m| m.as_str()),
        None => return None,
    };

    Some(keyword)
}

/// Supplementary function which returns a Vec of all keywords within the env
///
/// This function accepts the EnvContents struct available in the
/// [utils](../utils/index.html) module.
pub fn list_available_keywords(env: &EnvContents) -> Vec<&str> {
    let lines = env.contents.lines();
    let lines = lines.filter_map(|l| resolve_keyword(l));

    lines.collect()
}

/// Core function which performs all parsing and returns results
///
/// This function accepts and returns the structs available in the
/// [utils](../utils/index.html) module.
pub fn parse_env(env: &EnvContents, config: &Config) -> Result<EnvContents, String> {
    let lines = env.contents.lines();

    let mut parse_status = ParseStatus::Ignore;
    let mut keyword_found = false;

    let collected = lines.map(|line| {
        if line.is_empty() {
            parse_status = ParseStatus::Ignore;
            return String::from(line);
        }

        if let Some(keyword) = resolve_keyword(line) {
            if keyword == config.keyword {
                keyword_found = true;
                parse_status = ParseStatus::Active;
            } else {
                parse_status = ParseStatus::Inactive;
            }

            return String::from(line);
        };

        match parse_status {
            ParseStatus::Active => parse_as_active(line),
            ParseStatus::Inactive => parse_as_inactive(line),
            ParseStatus::Ignore => String::from(line),
        }
    });

    let collected: Vec<String> = collected.collect();
    let collected = collected.join("\n");
    // Ensure we have an end-of-file newline
    let collected = collected + "\n";

    match keyword_found {
        true => Ok(EnvContents::new(collected)),
        false => Err(format!(
            "keyword \"{}\" was not found in .env file",
            config.keyword
        )),
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

#[cfg(test)]
mod list_available_keywords_tests {
    use super::*;

    #[test]
    fn empty_if_no_keywords() {
        let provided = String::from(
            "
KEY=value
KEY=value

KEY=value
    ",
        );
        let env = EnvContents::new(provided.clone());
        let expected = vec![""; 0];

        assert_eq!(list_available_keywords(&env), expected)
    }

    #[test]
    fn returns_all_keywords() {
        let provided = String::from(
            "
# ++ a ++
# ++ b ++
KEY=value
KEY=value

# ++ c ++
KEY=value
    ",
        );
        let env = EnvContents::new(provided.clone());
        let expected = vec!["a", "b", "c"];

        assert_eq!(list_available_keywords(&env), expected)
    }
}

#[cfg(test)]
mod parse_env_tests {
    use super::*;

    #[test]
    fn err_if_keyword_not_found() {
        let provided = String::from(
            "
KEY=value
KEY=value

KEY=value
    ",
        );
        let env = EnvContents::new(provided.clone());
        let args = vec![String::from("_"), String::from("keyword")];
        let config = Config::new_from_args(args.into_iter()).unwrap();

        assert_eq!(
            parse_env(&env, &config),
            Err(String::from(
                "keyword \"keyword\" was not found in .env file"
            ))
        );
    }

    #[test]
    fn comment_out_non_matches() {
        let provided = String::from(
            "
# ++ a ++
KEY=value
# ++ b ++
KEY=value

# ++ c ++
KEY=value
",
        );
        let env = EnvContents::new(provided);
        let args = vec![String::from("_"), String::from("b")];
        let config = Config::new_from_args(args.into_iter()).unwrap();

        let expected = String::from(
            "
# ++ a ++
# KEY=value
# ++ b ++
KEY=value

# ++ c ++
# KEY=value
",
        );
        assert_eq!(parse_env(&env, &config), Ok(EnvContents::new(expected)));
    }

    #[test]
    fn leave_matches_if_uncommented() {
        let provided = String::from(
            "
# ++ a ++
KEY=value
# ++ b ++
KEY=value

# ++ c ++
KEY=value
",
        );
        let env = EnvContents::new(provided);
        let args = vec![String::from("_"), String::from("b")];
        let config = Config::new_from_args(args.into_iter()).unwrap();

        let expected = String::from(
            "
# ++ a ++
# KEY=value
# ++ b ++
KEY=value

# ++ c ++
# KEY=value
",
        );
        assert_eq!(parse_env(&env, &config), Ok(EnvContents::new(expected)));
    }

    #[test]
    fn uncomment_matches() {
        let provided = String::from(
            "
# ++ a ++
# KEY=value
# ++ b ++
# KEY=value

# ++ c ++
KEY=value
",
        );
        let env = EnvContents::new(provided);
        let args = vec![String::from("_"), String::from("b")];
        let config = Config::new_from_args(args.into_iter()).unwrap();

        let expected = String::from(
            "
# ++ a ++
# KEY=value
# ++ b ++
KEY=value

# ++ c ++
# KEY=value
",
        );
        assert_eq!(parse_env(&env, &config), Ok(EnvContents::new(expected)));
    }
}

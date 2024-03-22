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
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORD_REGEX: Regex = Regex::new(r"^# \+\+ ([0-9A-Za-z_-]{1,100})").unwrap();
    static ref VAR_REGEX: Regex = Regex::new(r"^# ?([[:word:]]+=\S*)").unwrap();
}

#[derive(PartialEq)]
enum ParseStatus {
    Active,
    Inactive,
    Ignore,
}

fn parse_as_active(line: &str) -> String {
    match VAR_REGEX.captures(line).map(|caps| caps.extract()) {
        Some((_, [var])) => String::from(var),
        _ => String::from(line),
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
    match KEYWORD_REGEX.captures(line).map(|caps| caps.extract()) {
        Some((_, [keyword])) => Some(keyword),
        _ => None,
    }
}

/// Supplementary function which returns a Vec of all keywords within the env
///
/// This function accepts the EnvContents struct available in the
/// [utils](../utils/index.html) module.
pub fn list_available_keywords(env: &EnvContents) -> Vec<&str> {
    let mut cache = HashMap::new();

    for line in env.contents.lines() {
        if let Some(keyword) = resolve_keyword(line) {
            cache.insert(keyword, 1);
        }
    }

    cache.into_keys().collect()
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
    fn removes_hash_from_var_variant_1() {
        assert_eq!(parse_as_active("# VAR=true"), "VAR=true");
    }

    #[test]
    fn removes_hash_from_var_variant_2() {
        assert_eq!(
            parse_as_active("# TEST_VAR_ENTRY=some??weird__43££combo*~*😃*929100"),
            "TEST_VAR_ENTRY=some??weird__43££combo*~*😃*929100"
        );
    }

    #[test]
    fn removes_hash_from_complex_var() {
        assert_eq!(
            parse_as_active("# 0varl337=123-yrHks~\""),
            "0varl337=123-yrHks~\""
        );
    }

    #[test]
    fn keeps_hash_from_comment() {
        assert_eq!(
            parse_as_active("# this should be VAR=true"),
            "# this should be VAR=true"
        );
        assert_eq!(parse_as_active("# COMMENT-true"), "# COMMENT-true");
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
    fn none_if_invalid_formatted_variant_1() {
        assert_eq!(resolve_keyword("#++ keyword"), None);
    }

    #[test]
    fn none_if_invalid_formatted_variant_2() {
        assert_eq!(resolve_keyword("## ++ keyword ++"), None);
    }

    #[test]
    fn word_if_formatted_variant_1() {
        assert_eq!(resolve_keyword("# ++ keyword ++"), Some("keyword"));
    }

    #[test]
    fn word_if_formatted_variant_2() {
        assert_eq!(resolve_keyword("# ++ my-env ++"), Some("my-env"));
    }

    #[test]
    fn word_if_formatted_variant_3() {
        assert_eq!(
            resolve_keyword("# ++ prod_environment ++"),
            Some("prod_environment")
        );
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
        let env = EnvContents::new(provided);
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
        let env = EnvContents::new(provided);
        let mut expected = vec!["a", "b", "c"];
        let mut result = list_available_keywords(&env);

        // allow default sorting - if they're the same there'll be a match
        expected.sort();
        result.sort();

        assert_eq!(result, expected)
    }

    #[test]
    fn dedup_keywords() {
        let provided = String::from(
            "
# ++ a ++
# ++ a ++
# ++ a ++
# ++ b ++
# ++ b ++
KEY=value
KEY=value


# ++ c ++
# ++ c ++
KEY=value
    ",
        );
        let env = EnvContents::new(provided);
        let mut expected = vec!["a", "b", "c"];
        let mut result = list_available_keywords(&env);

        // allow default sorting - if they're the same there'll be a match
        expected.sort();
        result.sort();

        assert_eq!(result, expected)
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
        let env = EnvContents::new(provided);
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

//! Helper functions and data types
//!
//! This module provides business-logic agnostic helpers
//! which can be used throughout the cenv codebase.
//!
//!

use std::fs;
use std::io::Write;

/// Configuration of the current domain
#[derive(PartialEq, Debug)]
pub struct Config {
    pub keyword: String,
}

impl Config {
    /// Returns Result with error on keyword being an empty string
    pub fn new(keyword: &str) -> Result<Config, &'static str> {
        let keyword = match keyword {
            "" => return Err("Keyword missing"),
            word => word,
        };

        Ok(Config {
            keyword: String::from(keyword),
        })
    }
    /// Accepts a list of arguments, usually an [Args][std::env::Args] struct
    /// sourced from the [std::env::args] function.
    pub fn new_from_args<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
        // ignore first arg (program name)
        args.next();

        let keyword = match args.next() {
            Some(word) => word,
            None => return Err("Keyword missing"),
        };

        Ok(Config { keyword })
    }
}

/// Details around the content to be parsed
#[derive(PartialEq, Debug)]
pub struct EnvContents {
    pub contents: String,
}

impl EnvContents {
    pub fn new(contents: String) -> EnvContents {
        EnvContents { contents }
    }
}

/// Reads .env file in execution scope
pub fn read_env_file() -> Result<EnvContents, &'static str> {
    let contents = match fs::read_to_string(".env") {
        Ok(w) => w,
        Err(_) => return Err("Unable to read .env file"),
    };
    Ok(EnvContents { contents })
}

/// Writes to the .env file in execution scope
pub fn write_env_file(env: &EnvContents) -> Result<(), String> {
    let mut file = match fs::File::create(".env") {
        Ok(f) => f,
        Err(e) => return Err(format!("Unable to write .env file - {}", e)),
    };

    match file.write_all(env.contents.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Unable to write .env file - {}", e)),
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;
    #[test]
    fn err_on_missing_keyword() {
        let args = vec![String::from("")];
        let result = Config::new_from_args(args.into_iter());
        assert_eq!(result, Err("Keyword missing"))
    }
    #[test]
    fn returns_populated_config() {
        let args = vec![String::from(""), String::from("testing")];
        let result = Config::new_from_args(args.into_iter()).unwrap();
        assert_eq!(
            result,
            Config {
                keyword: String::from("testing")
            }
        )
    }
}

#[cfg(test)]
mod env_contents_tests {
    use super::*;

    #[test]
    fn includes_correct_data() {
        let e = EnvContents::new(String::from("testing"));

        assert_eq!(
            e,
            EnvContents {
                contents: String::from("testing")
            }
        )
    }
}

#[cfg(test)]
mod read_env_file_tests {
    use super::*;

    fn setup() {
        let contents = String::from(
            "# ++ one ++
# TEST_A=1
# TEST_B=1

# ++ two ++
# TEST_A=2
# TEST_B=2

# ++ three ++
TEST_A=3
TEST_B=3
",
        );
        write_env_file(&EnvContents::new(contents)).unwrap();
    }

    #[test]
    fn does_not_error() {
        setup();
        let result = read_env_file();
        assert_ne!(result, Err("Unable to read .env file"));
    }

    #[test]
    fn returns_expected_content() {
        setup();
        let result = read_env_file();
        let contents = String::from(
            "# ++ one ++
# TEST_A=1
# TEST_B=1

# ++ two ++
# TEST_A=2
# TEST_B=2

# ++ three ++
TEST_A=3
TEST_B=3
",
        );
        assert_eq!(result, Ok(EnvContents { contents }));
    }
}

#[cfg(test)]
mod write_env_file_tests {
    use super::*;

    #[test]
    fn does_not_error() {
        let contents = String::from(
            "# ++ one ++
# TEST_A=1
# TEST_B=1

# ++ two ++
# TEST_A=2
# TEST_B=2

# ++ three ++
TEST_A=3
TEST_B=3
",
        );
        let result = write_env_file(&EnvContents::new(contents));
        assert_eq!(result, Ok(()));
    }
}

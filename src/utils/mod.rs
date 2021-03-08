use std::fs;
use std::io::Write;

#[derive(PartialEq)]
pub enum ParseStatus {
    Active,
    Inactive,
    Ignore,
}

#[derive(PartialEq, Debug)]
pub struct Config {
    pub keyword: String,
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, &'static str>
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

#[derive(PartialEq, Debug)]
pub struct EnvContents {
    pub contents: String,
}

impl EnvContents {
    pub fn new(contents: String) -> EnvContents {
        EnvContents { contents }
    }
}

pub fn read_env_file() -> Result<EnvContents, &'static str> {
    let contents = match fs::read_to_string(".env") {
        Ok(w) => w,
        Err(_) => return Err("Unable to read .env file"),
    };
    Ok(EnvContents { contents })
}

pub fn write_to_file(env: EnvContents) -> Result<(), String> {
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
        let result = Config::new(args.into_iter());
        assert_eq!(result, Err("Keyword missing"))
    }
    #[test]
    fn returns_populated_config() {
        let args = vec![String::from(""), String::from("testing")];
        let result = Config::new(args.into_iter()).unwrap();
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

    #[test]
    fn does_not_error() {
        let result = read_env_file();
        assert_ne!(result, Err("Unable to read .env file"));
    }

    #[test]
    fn returns_expected_content() {
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
mod write_to_file_tests {
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
        let result = write_to_file(EnvContents::new(contents));
        assert_eq!(result, Ok(()));
    }
}

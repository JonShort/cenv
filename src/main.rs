use std::env;
use std::process;

use cenv::parser;
use cenv::utils;

fn main() {
    let config = utils::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let env = utils::read_env_file().unwrap_or_else(|err| {
        eprintln!("Problem reading .env file: {}", err);
        process::exit(1);
    });

    let new_env = parser::parse_env(env, &config);

    match utils::write_to_file(new_env) {
        Ok(_) => println!("Updated .env to {}", &config.keyword),
        Err(e) => {
            eprintln!("Problem reading .env file: {}", e);
            process::exit(1);
        }
    }
}

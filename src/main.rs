use std::env;
use std::process;

use cenv_core::parser;
use cenv_core::utils;

fn main() {
    let config = utils::Config::new_from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let env = utils::read_env_file().unwrap_or_else(|err| {
        eprintln!("Problem reading .env file: {}", err);
        process::exit(1);
    });

    let new_env = match parser::parse_env(&env, &config) {
        Ok(env) => env,
        Err(e) => {
            eprintln!("Problem parsing env: {}", e);
            process::exit(1);
        }
    };

    match utils::write_env_file(&new_env) {
        Ok(_) => println!("Updated .env to {}", &config.keyword),
        Err(e) => {
            eprintln!("Problem reading .env file: {}", e);
            process::exit(1);
        }
    }
}

use std::env;
use std::process;

use cenv_core::parser;
use cenv_core::utils;

fn print_keywords(env: &utils::EnvContents) {
    println!("Available keywords:");

    for k in parser::list_available_keywords(env).into_iter() {
        println!("- {}", k);
    }
}

fn main() {
    let env = utils::read_env_file().unwrap_or_else(|err| {
        eprintln!("Problem reading .env file: {}", err);
        process::exit(1);
    });

    let config = utils::Config::new_from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        print_keywords(&env);
        process::exit(1);
    });

    let new_env = parser::parse_env(&env, &config).unwrap_or_else(|err| {
        eprintln!("Problem parsing env: {}", err);
        print_keywords(&env);
        process::exit(1);
    });

    match utils::write_env_file(&new_env) {
        Ok(_) => println!("Updated .env to {}", &config.keyword),
        Err(e) => {
            eprintln!("Problem reading .env file: {}", e);
            process::exit(1);
        }
    }
}

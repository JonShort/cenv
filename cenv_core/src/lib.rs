//! A library exposing modules to perform the business
//! logic around changing env vars, and also some utility
//! functions and data types.

#[macro_use]
extern crate lazy_static;
pub mod parser;
pub mod utils;

pub use parser::parse_env;
pub use utils::{read_env_file, write_env_file, Config, EnvContents};

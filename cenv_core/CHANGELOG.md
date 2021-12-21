# Changelog
All notable changes to cenv_core will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased
### Changed
- Target [rust 2021](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html#rust-2021)
- Comments are now valid within "cenv" blocks and will be ignored by the parser
- `list_available_keywords` now de-deuplicates listed keywords

## 0.2.0 - 2021-08-02
### Added
- `list_available_keywords` method added - allows consuming programs to give better hints / info around which keywords are available to the user

### Changed
- Made `resolve_keyword` method public - consumers may find having the ability to parse a single line useful
- `regex` dependency bumped from `1.4` to `1.5.4`

## 0.1.0 - 2021-06-19
### Added
- Initial parser methods
- Initial utils methods & data structures

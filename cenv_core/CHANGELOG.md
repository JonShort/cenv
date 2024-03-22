# Changelog
All notable changes to cenv_core will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## 1.0.0 - 2024-03-22
### Changed
- [BREAKING] Keyword line formatting is now stricter, e.g. `##++ thing` would previously match, now single comment & space are required, e.g. `# ++ thing`
- [BREAKING] The env var regex now expects env var names to follow [the UNIX-style standard for environment variables](https://pubs.opengroup.org/onlinepubs/7908799/xbd/envvar.html#:~:text=Environment%20variable%20names%20used%20by,the%20presence%20of%20such%20names.), but allows the following `0-9`, `A-Z`, `a-z`, `_`
- Keywords can now contain dashes
- Updated the [regex](https://crates.io/crates/regex) dependency to 1.10.x

## 0.3.1 - 2023-01-15
### Changed
- [regex](https://crates.io/crates/regex) crate dependency bumped to [1.7.1](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md#171-2023-01-09)

## 0.3.0 - 2021-12-21
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

# Changelog
All notable changes to this cenv_core will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- `list_available_keywords` method added - allows consuming programs to give better hints / info around which keywords are available to the user

### Changed
- Made `resolve_keyword` method public - consumers may find having the ability to parse a single line useful
- `regex` dependency bumped from `1.4` to `1.5.4`

## [0.1.0] - 2021-06-19
### Added
- Initial parser methods
- Initial utils methods & data structures

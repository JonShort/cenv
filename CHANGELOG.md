# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Binary size reduced following optimisations from [min-sized-rust](https://github.com/johnthagen/min-sized-rust) guide

## [1.2.0] - 2021-12-21
### Changed
- Target [rust 2021](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html#rust-2021)
- Comments are now valid within "cenv" blocks and will be ignored
- The keywords listed when an invalid choice is made are now de-deuplicated

## [1.1.0] - 2021-08-02
### Added
- Available keywords are now listed to the user when invalid or no keyword provided

## [1.0.0] - 2021-05-12
### Added
- Alert and exit if keyword doesn't exist within file

## [0.0.1] - 2021-03-11
### Added
- MVP functionality

[Unreleased]: https://github.com/JonShort/cenv/compare/v1.2.0...HEAD
[1.2.0]: https://github.com/JonShort/cenv/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/JonShort/cenv/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/JonShort/cenv/compare/v0.0.1...v1.0.0
[0.0.1]: https://github.com/jonshort/cenv/releases/tag/v0.0.1

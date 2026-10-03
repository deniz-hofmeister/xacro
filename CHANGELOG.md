# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Releases up to and including 0.1.3 predate this file.

## [Unreleased]

### Added

- `process_str(source, base_dir, options)`: expands xacro source text,
  resolving relative includes against `base_dir`. `XacroOptions` carries
  `$(arg)` values and `$(find)` package roots; the expansion does not read
  them yet.
- Macro expansion: `xacro:macro` definitions are collected, calls are
  expanded with their parameters substituted, and the definitions are removed
  from the output. The expansion still prints its intermediate documents to
  stdout.
- `XacroError::MissingParameter { macro_name, param }`.
- The in-scope tests of the upstream ROS 2 xacro suite, ported with their
  fixtures (BSD-3-Clause). They specify the target feature set; most fail
  until their features land.

### Changed

- `XacroError::Macro` is renamed to `XacroError::UndefinedMacro`.

### Removed

- The unused `hashbrown` dependency.

[Unreleased]: https://github.com/deniz-hofmeister/xacro/compare/7949b1b...HEAD

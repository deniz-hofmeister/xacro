# xacro

[![Crates.io](https://img.shields.io/crates/v/xacro.svg)](https://crates.io/crates/xacro)
[![Documentation](https://docs.rs/xacro/badge.svg)](https://docs.rs/xacro)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![tests](https://github.com/deniz-hofmeister/xacro/actions/workflows/tests.yml/badge.svg?branch=master)](https://github.com/deniz-hofmeister/xacro/actions/workflows/tests.yml)

A xml preprocessor for xacro files to generate URDF files

[API reference](https://docs.rs/xacro) · [Changelog](CHANGELOG.md)

## WIP
Placeholder for the future xacro preprocessing tool.
Reference: https://github.com/ros/xacro/wiki

## TODO
The following functionality shall be implemented:
- [ ] macro
- [X] include
- [ ] insert_block
- [X] property
- [ ] element
- [ ] if
- [ ] unless
- [ ] loop

These seem like the core functionalities required for proper xacro file handling.

The following might be implemented:
- [ ] load_yaml, unsure if yaml & xacro split is the right way to do things. Why not have a xacro with the properties?
- [ ] eval-comments, not sure how useful this will be vs the amount of effort required to implement this.

Fundamentally these feel like scope creep or nice-to-haves

If needed, the following alternative can be developed
- [ ] Rust-based evaluation / mathematical expressions, to match the python evaluation

## Non-Goals

The following are outside this crate's scope:

- Python-based evaluation
- Rospack-based evaluation
- Windows compatibility

This package is meant to be as dependency free as possible, so no Python or
ROS dependencies.

## AI-Assisted Development

This project uses AI assistance. Assisted commits carry an
`Assisted-by: <Agent>:<model>` trailer. The maintainer reviews contributions
and takes responsibility for the code. See [AGENTS.md](AGENTS.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the workflow and verification gate,
and [SECURITY.md](SECURITY.md) for vulnerability reporting.

## License

[MIT](LICENSE). The upstream xacro test fixtures under
`tests/fixtures/ros_xacro/` are BSD-3-Clause; see their
[license](tests/fixtures/ros_xacro/LICENSE).

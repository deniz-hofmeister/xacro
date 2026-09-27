#![forbid(unsafe_code)]
// #![warn(clippy::pedantic)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_core)]

pub mod error;
pub mod features;
pub mod processor;
pub mod types;
pub mod utils;

pub use error::XacroError;
pub use processor::XacroProcessor;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Default)]
pub struct XacroOptions {
    /// Values for `$(arg name)`, as given by `name:=value` on the xacro command line
    pub args: HashMap<String, String>,
    /// Package root directories used to resolve `$(find package)`
    pub packages: HashMap<String, PathBuf>,
}

pub fn process_file<P: AsRef<std::path::Path>>(path: P) -> Result<String, XacroError> {
    let processor = XacroProcessor::new();
    processor.run(path)
}

/// Processes xacro source text, resolving relative includes against `base_dir`.
pub fn process_str<P: AsRef<Path>>(
    source: &str,
    base_dir: P,
    options: &XacroOptions,
) -> Result<String, XacroError> {
    // The pipeline does not support `$(arg)` or `$(find)` yet, so the options are unused for now
    let _ = options;
    XacroProcessor::new().run_str(source, base_dir.as_ref())
}

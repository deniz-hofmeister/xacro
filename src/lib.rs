#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_core)]

pub mod error;
pub mod features;
pub mod processor;
pub mod types;
pub mod utils;

pub use processor::XacroProcessor;

// Re-export commonly used types
pub use error::XacroError;

/// Main entry point for processing xacro files
pub fn process_file<P: AsRef<std::path::Path>>(path: P) -> Result<String, XacroError> {
    let processor = XacroProcessor::new();
    processor.process_file(path)
}

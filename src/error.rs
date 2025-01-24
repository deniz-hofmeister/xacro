#[derive(Debug, thiserror::Error)]
pub enum XacroError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("XML error: {0}")]
    Xml(#[from] xmltree::ParseError),

    #[error("Include error: {0}")]
    Include(String),

    #[error("Macro error: {0}")]
    Macro(String),
}

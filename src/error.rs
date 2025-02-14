#[derive(Debug, thiserror::Error)]
pub enum XacroError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("XML error: {0}")]
    Xml(#[from] xmltree::ParseError),

    #[error("Include error: {0}")]
    Include(String),

    #[error("Macro error: {0}")]
    UndefinedMacro(String),

    #[error("Missing parameter '{param}' in macro '{macro_name}'")]
    MissingParameter { macro_name: String, param: String },

    #[error("Macro error: {0}")]
    PropertyNotFound(String),

    #[error("XML write error: {0}")]
    XmlWrite(#[from] xmltree::Error),
}

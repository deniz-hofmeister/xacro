use crate::{error::XacroError, XacroProcessor};
use std::fs::File;

impl XacroProcessor {
    pub(super) fn parse_file<P: AsRef<std::path::Path>>(
        path: P
    ) -> Result<xmltree::Element, XacroError> {
        let file = std::fs::File::open(path)?;
        Ok(xmltree::Element::parse(file)?)
    }

    pub(super) fn serialize<P: AsRef<std::path::Path>>(
        xml: xmltree::Element,
        path: P,
    ) -> Result<String, XacroError> {
        let output_path = format!(
            "{}.urdf",
            path.as_ref().to_string_lossy().trim_end_matches(".xacro")
        );

        let mut file = File::create(&output_path)?;
        xml.write(&mut file)?;

        Ok(output_path)
    }
}

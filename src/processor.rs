use crate::{
    error::XacroError,
    features::{
        conditions::ConditionProcessor, includes::IncludeProcessor, loops::LoopProcessor,
        macros::MacroProcessor, properties::PropertyProcessor,
    },
};
use std::fs::File;

pub struct XacroProcessor {
    macros: MacroProcessor,
    properties: PropertyProcessor,
    conditions: ConditionProcessor,
    loops: LoopProcessor,
    includes: IncludeProcessor,
}

impl XacroProcessor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            includes: IncludeProcessor::new(),
            macros: MacroProcessor::new(),
            properties: PropertyProcessor::new(),
            conditions: ConditionProcessor::new(),
            loops: LoopProcessor::new(),
        }
    }

    pub fn run<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<String, XacroError> {
        let xml = XacroProcessor::parse_file(&path)?;

        // 2. Process features in order
        let xml = self.includes.process(xml)?;
        let xml = self.properties.process(xml)?;
        let xml = self.macros.process(xml)?;
        let xml = self.conditions.process(xml)?;
        let xml = self.loops.process(xml)?;

        XacroProcessor::serialize(xml, &path)
    }

    fn parse_file<P: AsRef<std::path::Path>>(path: P) -> Result<xmltree::Element, XacroError> {
        let file = std::fs::File::open(path)?;
        Ok(xmltree::Element::parse(file)?)
    }

    fn serialize<P: AsRef<std::path::Path>>(
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

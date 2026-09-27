use crate::{
    error::XacroError,
    features::{
        conditions::ConditionProcessor, includes::IncludeProcessor, loops::LoopProcessor,
        macros::MacroProcessor, properties::PropertyProcessor,
    },
};

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
        let xml = self.expand(xml, path.as_ref())?;
        XacroProcessor::serialize(xml, &path)
    }

    pub fn run_str(
        &self,
        source: &str,
        base_dir: &std::path::Path,
    ) -> Result<String, XacroError> {
        let xml = xmltree::Element::parse(source.as_bytes())?;
        // Includes resolve against the parent of the document path, so name a file inside base_dir
        let xml = self.expand(xml, &base_dir.join("string"))?;
        XacroProcessor::serialize_to_string(&xml)
    }

    fn expand(
        &self,
        xml: xmltree::Element,
        path: &std::path::Path,
    ) -> Result<xmltree::Element, XacroError> {
        let xml = self.includes.process(xml, path)?;
        let xml = self.properties.process(xml)?;
        let xml = self.macros.process(xml)?;
        let xml = self.conditions.process(xml)?;
        self.loops.process(xml)
    }
}

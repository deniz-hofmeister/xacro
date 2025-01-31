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

        // 2. Process features in order
        let xml = self.includes.process(xml)?;
        let xml = self.properties.process(xml)?;
        let xml = self.macros.process(xml)?;
        let xml = self.conditions.process(xml)?;
        let xml = self.loops.process(xml)?;

        XacroProcessor::serialize(xml, &path)
    }
}

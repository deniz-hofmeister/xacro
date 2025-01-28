use crate::{error::XacroError, features::*};

pub struct XacroProcessor {
    macros: macro_::MacroProcessor,
    properties: property::PropertyProcessor,
    conditions: condition::ConditionProcessor,
    loops: loop_::LoopProcessor,
}

impl XacroProcessor {
    pub fn new() -> Self {
        Self {
            includes: include::IncludeProcessor::new(),
            macros: macro_::MacroProcessor::new(),
            properties: property::PropertyProcessor::new(),
            conditions: condition::ConditionProcessor::new(),
            loops: loop_::LoopProcessor::new(),
        }
    }

    pub fn process_file<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<String, XacroError> {
        let xml = self.parse_file(path)?;

        // 2. Process features in order
        let xml = self.includes.process(xml)?;
        let xml = self.properties.process(xml)?;
        let xml = self.macros.process(xml)?;
        let xml = self.conditions.process(xml)?;
        let xml = self.loops.process(xml)?;

        self.serialize(xml)
    }
}

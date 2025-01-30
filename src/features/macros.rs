use crate::error::XacroError;
use xmltree::Element;

pub struct MacroProcessor {}

impl MacroProcessor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process(
        &self,
        xml: Element,
    ) -> Result<Element, XacroError> {
        Ok(xml)
    }
}

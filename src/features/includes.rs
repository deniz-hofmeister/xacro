use crate::error::XacroError;
use xmltree::Element;

pub struct IncludeProcessor {}

impl IncludeProcessor {
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

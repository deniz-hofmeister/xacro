use crate::error::XacroError;
use xmltree::Element;

pub struct LoopProcessor {}

impl LoopProcessor {
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

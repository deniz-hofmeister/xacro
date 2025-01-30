use crate::error::XacroError;
use xmltree::Element;

pub struct ElementsProcessor {}

impl ElementsProcessor {
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

use crate::error::XacroError;
use xmltree::Element;

pub struct PropertyProcessor {}

impl PropertyProcessor {
    #[allow(clippy::new_without_default)]
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

use crate::error::XacroError;
use xmltree::Element;

pub struct ConditionProcessor {}

impl ConditionProcessor {
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

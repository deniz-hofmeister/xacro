use crate::error::XacroError;
use xmltree::Element;

pub struct IncludeProcessor {}

impl IncludeProcessor {
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

#[cfg(test)]
mod tests;

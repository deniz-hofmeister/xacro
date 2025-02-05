use crate::error::XacroError;
use std::{fs, path::Path};
use xmltree::{Element, XMLNode::Element as NodeElement};

pub struct IncludeProcessor {}

impl IncludeProcessor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn process(
        &self,
        mut xml: Element,
        xml_path: &Path,
    ) -> Result<Element, XacroError> {
        IncludeProcessor::process_element(&mut xml, xml_path)?;
        Ok(xml)
    }

    fn process_element(
        element: &mut Element,
        xml_path: &Path,
    ) -> Result<(), XacroError> {
        let mut new_children = Vec::new();

        for child in &element.children {
            if let NodeElement(mut child_element) = child.clone() {
                if child_element.name == "include" {
                    if let Some(filename) = child_element.attributes.get("filename") {
                        let file_path = xml_path.parent().unwrap().join(filename);
                        let file_content = fs::read_to_string(file_path)?;
                        let mut included_element = Element::parse(file_content.as_bytes())?;
                        IncludeProcessor::process_element(&mut included_element, xml_path)?;
                        new_children.extend(included_element.children);
                    }
                } else {
                    IncludeProcessor::process_element(&mut child_element, xml_path)?;
                    new_children.push(NodeElement(child_element));
                }
            } else {
                new_children.push(child.clone());
            }
        }

        element.children = new_children;
        Ok(())
    }
}

#[cfg(test)]
mod tests;

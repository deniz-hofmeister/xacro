use crate::error::XacroError;
use std::collections::HashMap;
use xmltree::{
    Element,
    XMLNode::{Element as NodeElement, Text as TextElement},
};

pub struct PropertyProcessor {}

impl PropertyProcessor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn process(
        &self,
        mut xml: Element,
    ) -> Result<Element, XacroError> {
        let mut properties = HashMap::new();
        Self::collect_properties(&xml, &mut properties)?;
        Self::substitute_properties(&mut xml, &properties)?;
        Self::remove_property_elements(&mut xml);
        Ok(xml)
    }

    fn collect_properties(
        element: &Element,
        properties: &mut HashMap<String, String>,
    ) -> Result<(), XacroError> {
        if element.name == "property" {
            if let (Some(name), Some(value)) = (
                element.attributes.get("name"),
                element.attributes.get("value"),
            ) {
                properties.insert(name.clone(), value.clone());
            }
        }

        for child in &element.children {
            if let NodeElement(child_elem) = child {
                Self::collect_properties(child_elem, properties)?;
            }
        }

        Ok(())
    }

    pub(crate) fn substitute_properties(
        element: &mut Element,
        properties: &HashMap<String, String>,
    ) -> Result<(), XacroError> {
        for value in element.attributes.values_mut() {
            *value = Self::substitute_in_text(value, properties)?;
        }

        for child in &mut element.children {
            if let NodeElement(child_elem) = child {
                Self::substitute_properties(child_elem, properties)?;
            } else if let TextElement(text) = child {
                *text = Self::substitute_in_text(text, properties)?;
            }
        }

        Ok(())
    }

    fn substitute_in_text(
        text: &str,
        properties: &HashMap<String, String>,
    ) -> Result<String, XacroError> {
        let mut result = text.to_string();

        while let Some(start) = result.find("${") {
            if let Some(end) = result[start..].find('}') {
                let end = start + end + 1;
                let prop_name = &result[start + 2..end - 1];

                if let Some(value) = properties.get(prop_name) {
                    result.replace_range(start..end, value);
                } else {
                    return Err(XacroError::PropertyNotFound(prop_name.to_string()));
                }
            }
        }

        Ok(result)
    }

    fn remove_property_elements(element: &mut Element) {
        element.children.retain_mut(|child| {
            if let NodeElement(child_elem) = child {
                if child_elem.name == "property" {
                    return false;
                }
                Self::remove_property_elements(child_elem);
            }
            true
        });
    }
}

#[cfg(test)]
mod tests;

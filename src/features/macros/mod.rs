use crate::{
    error::XacroError,
    features::properties::PropertyProcessor,
    utils::{pretty_print_hashmap, pretty_print_xml},
};
use std::collections::HashMap;
use xmltree::{Element, XMLNode::Element as NodeElement};

#[derive(Debug, Clone)]
struct MacroDefinition {
    params: HashMap<String, Option<String>>,
    content: Element,
}

pub struct MacroProcessor {}

impl MacroProcessor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn process(
        &self,
        mut xml: Element,
    ) -> Result<Element, XacroError> {
        let mut macros = HashMap::new();
        println!("Input XML:");
        println!("{}", pretty_print_xml(&xml));

        Self::collect_macros(&xml, &mut macros)?;
        println!("Collected macros:");
        println!("{}", pretty_print_hashmap(&macros));

        Self::expand_macros(&mut xml, &macros)?;
        println!("Expanded XML:");
        println!("{}", pretty_print_xml(&xml));

        Self::remove_macro_definitions(&mut xml);
        println!("Output XML:");
        println!("{}", pretty_print_xml(&xml));
        Ok(xml)
    }

    fn collect_macros(
        element: &Element,
        macros: &mut HashMap<String, MacroDefinition>,
    ) -> Result<(), XacroError> {
        if Self::is_macro_definition(element) {
            if let (Some(name), Some(params)) = (
                element.attributes.get("name"),
                element.attributes.get("params"),
            ) {
                let params_map = Self::parse_params(params)?;
                let content = element.clone();
                macros.insert(
                    name.clone(),
                    MacroDefinition {
                        params: params_map,
                        content,
                    },
                );
            }
        }

        for child in &element.children {
            if let NodeElement(child_elem) = child {
                Self::collect_macros(child_elem, macros)?;
            }
        }

        Ok(())
    }

    fn parse_params(params_str: &str) -> Result<HashMap<String, Option<String>>, XacroError> {
        let mut params = HashMap::new();
        for param in params_str.split_whitespace() {
            if param.contains(":=") {
                let parts: Vec<&str> = param.split(":=").collect();
                if parts.len() == 2 {
                    params.insert(parts[0].to_string(), Some(parts[1].to_string()));
                }
            } else {
                params.insert(param.to_string(), None);
            }
        }
        Ok(params)
    }

    fn expand_macros(
        element: &mut Element,
        macros: &HashMap<String, MacroDefinition>,
    ) -> Result<(), XacroError> {
        let mut new_children = Vec::new();

        for child in core::mem::take(&mut element.children) {
            match child {
                NodeElement(mut child_elem) => {
                    if Self::is_macro_call(&child_elem) {
                        let macro_name = Self::get_macro_name(&child_elem)?;
                        if let Some(macro_def) = macros.get(&macro_name) {
                            let args = Self::collect_macro_args(&child_elem)?;
                            let expanded = Self::expand_macro_content(macro_def, &args)?;
                            new_children.extend(expanded.children);
                        } else {
                            return Err(XacroError::UndefinedMacro(macro_name));
                        }
                    } else {
                        Self::expand_macros(&mut child_elem, macros)?;
                        new_children.push(NodeElement(child_elem));
                    }
                }
                _ => new_children.push(child),
            }
        }

        element.children = new_children;
        Ok(())
    }

    fn expand_macro_content(
        macro_def: &MacroDefinition,
        args: &HashMap<String, String>,
    ) -> Result<Element, XacroError> {
        let mut content = macro_def.content.clone();

        for param_name in args.keys() {
            if !macro_def.params.contains_key(param_name) {
                return Err(XacroError::MissingParameter {
                    macro_name: content.name.clone(),
                    param: param_name.clone(),
                });
            }
        }

        let mut substitutions = HashMap::new();
        for (param_name, default_value) in &macro_def.params {
            let value = args
                .get(param_name)
                .cloned()
                .or_else(|| default_value.clone())
                .ok_or_else(|| XacroError::MissingParameter {
                    macro_name: content.name.clone(),
                    param: param_name.clone(),
                })?;
            substitutions.insert(param_name.clone(), value);
        }

        PropertyProcessor::substitute_properties(&mut content, &substitutions)?;

        Ok(content)
    }

    fn remove_macro_definitions(element: &mut Element) {
        element.children.retain_mut(|child| {
            if let NodeElement(child_elem) = child {
                if Self::is_macro_definition(child_elem) {
                    println!("Removing macro definition: {:?}", child_elem);
                    return false;
                }
                Self::remove_macro_definitions(child_elem);
            }
            true
        });
    }

    fn is_macro_call(element: &Element) -> bool {
        element.prefix.as_ref().is_some_and(|x| x == "xacro") && !element.name.eq("macro")
    }

    fn is_macro_definition(element: &Element) -> bool {
        element.prefix.as_ref().is_some_and(|x| x == "xacro") && element.name.eq("macro")
    }

    fn get_macro_name(element: &Element) -> Result<String, XacroError> {
        Ok(element.name.clone())
    }

    fn collect_macro_args(element: &Element) -> Result<HashMap<String, String>, XacroError> {
        Ok(element.attributes.clone())
    }
}

#[cfg(test)]
mod tests;

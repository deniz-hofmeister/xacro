use std::fs;
use std::path::{Path, PathBuf};
use xmltree::{Element, XMLNode};

#[derive(Debug)]
pub struct XacroProcessor {
    base_path: PathBuf,
}

impl XacroProcessor {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    /// Process a xacro file and return the expanded XML
    pub fn process_file<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> Result<Element, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let mut xml = Element::parse(content.as_bytes())?;
        self.process_includes(&mut xml)?;
        Ok(xml)
    }

    /// Recursively process includes in the XML tree
    fn process_includes(&self, element: &mut Element) -> Result<(), Box<dyn std::error::Error>> {
        // We need to collect children that need replacement first
        let mut includes = Vec::new();

        // Find all include elements
        for (index, child) in element.children.iter().enumerate() {
            if let XMLNode::Element(child_elem) = child {
                if self.is_xacro_include(child_elem) {
                    includes.push((index, self.get_include_path(child_elem)?));
                }
            }
        }

        // Process includes (in reverse order to maintain correct indices)
        for (index, path) in includes.into_iter().rev() {
            // Read and parse the included file
            let included_content = fs::read_to_string(&path)?;
            let mut included_xml = Element::parse(included_content.as_bytes())?;

            // Recursively process includes in the included file
            self.process_includes(&mut included_xml)?;

            // Replace the include element with the included content
            element.children.remove(index);

            // If the included XML has a root element, add its children
            // Otherwise, add the entire included XML
            if included_xml.name == "robot" {
                element
                    .children
                    .splice(index..index, included_xml.children.into_iter());
            } else {
                element
                    .children
                    .insert(index, XMLNode::Element(included_xml));
            }
        }

        // Process includes in remaining children
        for child in &mut element.children {
            if let XMLNode::Element(child_elem) = child {
                self.process_includes(child_elem)?;
            }
        }

        Ok(())
    }

    /// Check if an element is a xacro include
    fn is_xacro_include(&self, element: &Element) -> bool {
        element.name == "include" && element.namespace.as_deref() == Some("xacro")
            || element.name == "xacro:include"
    }

    /// Get the full path for an include element
    fn get_include_path(&self, element: &Element) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let filename = element
            .attributes
            .get("filename")
            .ok_or("Missing filename attribute in include")?;

        Ok(self.base_path.join(filename))
    }
}

/// Helper function to write the processed XML to a string
pub fn write_xml(element: &Element) -> Result<String, Box<dyn std::error::Error>> {
    let mut output = Vec::new();
    element.write(&mut output)?;
    Ok(String::from_utf8(output)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_include_processing() -> Result<(), Box<dyn std::error::Error>> {
        // Create temporary directory for test files
        let temp_dir = TempDir::new()?;

        // Create included file
        let included_content = r#"<link name="included_link">
            <inertial>
                <mass value="1.0"/>
                <origin xyz="0 0 0" rpy="0 0 0"/>
                <inertia ixx="1" ixy="0" ixz="0" iyy="1" iyz="0" izz="1"/>
            </inertial>
        </link>"#;

        let included_path = temp_dir.path().join("included.xacro");
        let mut file = File::create(&included_path)?;
        file.write_all(included_content.as_bytes())?;

        // Create main file
        let main_content = r#"<?xml version="1.0"?>
        <robot name="test_robot">
            <xacro:include filename="included.xacro"/>
        </robot>"#;

        let main_path = temp_dir.path().join("main.xacro");
        let mut file = File::create(&main_path)?;
        file.write_all(main_content.as_bytes())?;

        // Process the files
        let processor = XacroProcessor::new(temp_dir.path());
        let result = processor.process_file(&main_path)?;

        // Convert to string for comparison
        let output = write_xml(&result)?;

        // Basic verification
        assert!(output.contains("included_link"));
        assert!(output.contains("mass value=\"1.0\""));

        Ok(())
    }
}

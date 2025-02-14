#[cfg(test)]
mod macro_tests {
    use crate::{
        features::macros::MacroProcessor,
        utils::{pretty_print_xml, print_diff},
        XacroProcessor,
    };
    use log::error;
    use std::path::Path;

    #[test]
    fn test_macro_basic() {
        env_logger::try_init().ok();
        let macro_processor = MacroProcessor::new();
        let path = Path::new("tests/data/macro_test.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected = XacroProcessor::parse_file("tests/data/macro_test_expected.xacro").unwrap();

        let result = macro_processor.process(data);

        match result {
            Ok(actual) => {
                let expected_str = pretty_print_xml(&expected);
                let actual_str = pretty_print_xml(&actual);

                if actual != expected {
                    error!("\nXML Difference (actual vs expected):");
                    print_diff(&actual_str, &expected_str);
                    panic!("XML documents are different");
                }
            }
            Err(e) => {
                error!("Processing failed: {:?}", e);
                panic!("Macro processing failed");
            }
        }
    }
}

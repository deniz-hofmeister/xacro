#[cfg(test)]
mod macro_tests {
    use crate::{features::macros::MacroProcessor, utils::assert_xml_equal, XacroProcessor};
    use std::path::Path;

    #[test]
    fn test_macro_basic() {
        env_logger::try_init().ok();
        let macro_processor = MacroProcessor::new();
        let path = Path::new("tests/data/macro_test.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected = XacroProcessor::parse_file("tests/data/macro_test_expected.xacro").unwrap();

        let result = macro_processor.process(data);

        assert_xml_equal(result, expected)
    }

    #[test]
    fn test_macro_default() {
        env_logger::try_init().ok();
        let macro_processor = MacroProcessor::new();
        let path = Path::new("tests/data/macro_test_default.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected =
            XacroProcessor::parse_file("tests/data/macro_test_default_expected.xacro").unwrap();

        let result = macro_processor.process(data);

        assert_xml_equal(result, expected)
    }
    #[test]
    fn test_macro_nested() {
        env_logger::try_init().ok();
        let macro_processor = MacroProcessor::new();
        let path = Path::new("tests/data/macro_test_nested.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected =
            XacroProcessor::parse_file("tests/data/macro_test_nested_expected.xacro").unwrap();

        let result = macro_processor.process(data);

        assert_xml_equal(result, expected)
    }
}

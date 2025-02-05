#[cfg(test)]
mod macro_tests {
    use crate::{features::macros::MacroProcessor, XacroProcessor};
    #[test]
    fn test_include_basic() {
        let macro_processor = MacroProcessor::new();
        let data = XacroProcessor::parse_file("tests/data/include_base.xacro").unwrap();
        let expected = XacroProcessor::parse_file("tests/data/include_base_expected.urdf").unwrap();

        let result = macro_processor.process(data);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}

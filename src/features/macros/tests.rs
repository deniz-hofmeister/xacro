#[cfg(test)]
mod macro_tests {
    use crate::features::macros::{cases, MacroProcessor};
    #[test]
    fn test_include_basic() {
        let macro_processor = MacroProcessor::new();
        let data = xmltree::Element::parse(cases::TEST_INCLUDE_BASE_COMPONENT).unwrap();
        let expected = xmltree::Element::parse(cases::TEST_INCLUDE_BASE_COMPONENT).unwrap();
        let result = macro_processor.process(data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}

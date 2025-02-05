#[cfg(test)]
mod macro_tests {
    use crate::features::macros::MacroProcessor;
    #[test]
    fn test_include_basic() {
        let macro_processor = MacroProcessor::new();
        let result = macro_processor.process(data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}

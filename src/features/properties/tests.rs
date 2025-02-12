#[cfg(test)]
mod property_tests {
    use crate::{features::properties::PropertyProcessor, XacroProcessor};
    use log::error;
    use std::path::Path;

    #[test]
    fn test_property_basic() {
        env_logger::try_init().ok();
        let property_processor = PropertyProcessor::new();
        let path = Path::new("tests/data/property_test_base.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected =
            XacroProcessor::parse_file("tests/data/property_test_expected.xacro").unwrap();

        let result = property_processor.process(data);

        if result.is_err() {
            error!("{:?}", result);
        }

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_property_nested() {
        env_logger::try_init().ok();
        let property_processor = PropertyProcessor::new();
        let path = Path::new("tests/data/property_test_nested.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected =
            XacroProcessor::parse_file("tests/data/property_test_nested_expected.xacro").unwrap();

        let result = property_processor.process(data);

        if result.is_err() {
            error!("{:?}", result);
        }

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}

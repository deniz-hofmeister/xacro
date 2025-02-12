#[cfg(test)]
mod property_tests {
    use crate::{features::properties::PropertyProcessor, XacroProcessor};
    use log::error;
    use std::path::Path;

    #[test]
    fn test_property_basic() {
        env_logger::try_init().ok();
        let property_processor = PropertyProcessor::new();
        let path = Path::new("tests/data/property_test.xacro");
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

    #[test]
    fn test_property_multiple() {
        env_logger::try_init().ok();
        let property_processor = PropertyProcessor::new();
        let path = Path::new("tests/data/property_test_multiple.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected =
            XacroProcessor::parse_file("tests/data/property_test_multiple_expected.xacro").unwrap();

        let result = property_processor.process(data);

        if result.is_err() {
            error!("{:?}", result);
        }

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_property_multiple_out_of_order() {
        env_logger::try_init().ok();
        let property_processor = PropertyProcessor::new();
        let path = Path::new("tests/data/property_test_multiple_out_of_order.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected = XacroProcessor::parse_file(
            "tests/data/property_test_multiple_out_of_order_expected.xacro",
        )
        .unwrap();

        let result = property_processor.process(data);

        if result.is_err() {
            error!("{:?}", result);
        }

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_property_attributes() {
        env_logger::try_init().ok();
        let property_processor = PropertyProcessor::new();
        let path = Path::new("tests/data/property_test_attributes.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected =
            XacroProcessor::parse_file("tests/data/property_test_attributes_expected.xacro")
                .unwrap();

        let result = property_processor.process(data);

        if result.is_err() {
            error!("{:?}", result);
        }

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_property_multi_substitution() {
        env_logger::try_init().ok();
        let property_processor = PropertyProcessor::new();
        let path = Path::new("tests/data/property_test_multi_substitution.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected = XacroProcessor::parse_file(
            "tests/data/property_test_multi_substitution_expected.xacro",
        )
        .unwrap();

        let result = property_processor.process(data);

        if result.is_err() {
            error!("{:?}", result);
        }

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}

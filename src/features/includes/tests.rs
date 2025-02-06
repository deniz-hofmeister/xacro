#[cfg(test)]
mod macro_tests {
    use crate::{features::includes::IncludeProcessor, XacroProcessor};
    use log::error;
    use std::path::Path;

    #[test]
    fn test_include_basic() {
        let macro_processor = IncludeProcessor::new();
        let path = Path::new("tests/data/include_test.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected =
            XacroProcessor::parse_file("tests/data/include_test_expected.xacro").unwrap();

        let result = macro_processor.process(data, path);

        if result.is_err() {
            error!("{:?}", result);
        }

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_include_multi() {
        let macro_processor = IncludeProcessor::new();
        let path = Path::new("tests/data/include_test_multi_base.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected =
            XacroProcessor::parse_file("tests/data/include_test_multi_expected.xacro").unwrap();

        let result = macro_processor.process(data, path);

        if result.is_err() {
            println!("{:?}", result);
        }

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_include_nested() {
        let macro_processor = IncludeProcessor::new();
        let path = Path::new("tests/data/include_test_nested_base.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected =
            XacroProcessor::parse_file("tests/data/include_test_nested_expected.xacro").unwrap();

        let result = macro_processor.process(data, path);

        if result.is_err() {
            println!("{:?}", result);
        }

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}

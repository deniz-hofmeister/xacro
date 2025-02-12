#[cfg(test)]
mod include_tests {
    use crate::{features::includes::IncludeProcessor, XacroProcessor};
    use log::error;
    use std::path::Path;

    #[test]
    fn test_include_basic() {
        env_logger::try_init().ok();
        let include_processor = IncludeProcessor::new();
        let path = Path::new("tests/data/include_test.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected =
            XacroProcessor::parse_file("tests/data/include_test_expected.xacro").unwrap();

        let result = include_processor.process(data, path);

        if result.is_err() {
            error!("{:?}", result);
        }

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_include_multi() {
        env_logger::try_init().ok();
        let include_processor = IncludeProcessor::new();
        let path = Path::new("tests/data/include_test_multi_base.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected =
            XacroProcessor::parse_file("tests/data/include_test_multi_expected.xacro").unwrap();

        let result = include_processor.process(data, path);

        if result.is_err() {
            println!("{:?}", result);
        }

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_include_nested() {
        env_logger::try_init().ok();
        let include_processor = IncludeProcessor::new();
        let path = Path::new("tests/data/include_test_nested_base.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected =
            XacroProcessor::parse_file("tests/data/include_test_nested_expected.xacro").unwrap();

        let result = include_processor.process(data, path);

        if result.is_err() {
            println!("{:?}", result);
        }

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_include_subdirectory() {
        env_logger::try_init().ok();
        let include_processor = IncludeProcessor::new();
        let path = Path::new("tests/data/include_test_directory.xacro");
        let data = XacroProcessor::parse_file(path).unwrap();
        let expected =
            XacroProcessor::parse_file("tests/data/include_test_directory_expected.xacro").unwrap();

        let result = include_processor.process(data, path);

        if result.is_err() {
            println!("{:?}", result);
        }

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}

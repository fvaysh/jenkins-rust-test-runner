#[cfg(test)]
mod tests {
    use super::super::data_converter::DataConverter;

    #[test]
    fn test_json_to_csv() {
        let json_input = r#"{"name":"John","age":30}"#;
        let csv_output = DataConverter::json_to_csv(json_input);
        assert!(csv_output.contains("name:John"));
    }

    #[test]
    fn test_csv_to_json() {
        let csv_input = "name:John\nage:30";
        let json_output = DataConverter::csv_to_json(csv_input);
        assert!(json_output.contains("name:John"));
    }
}

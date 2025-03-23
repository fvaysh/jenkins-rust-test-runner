#[cfg(test)]
mod tests {
    use super::super::log_processor::LogProcessor;

    #[test]
    fn test_filter_errors() {
        let logs = vec![
            "INFO: All systems go".to_string(),
            "ERROR: Something went wrong".to_string(),
            "WARNING: Disk space low".to_string(),
        ];
        let errors = LogProcessor::filter_errors(&logs);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_count_warnings() {
        let logs = vec![
            "INFO: Starting up".to_string(),
            "WARNING: Low memory".to_string(),
            "WARNING: CPU Overload".to_string(),
        ];
        let count = LogProcessor::count_warnings(&logs);
        assert_eq!(count, 2);
    }
}

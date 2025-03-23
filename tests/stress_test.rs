use super::super::log_processor::LogProcessor;

#[test]
fn test_large_log_processing() {
    let mut logs = vec![];
    for i in 0..1_000_000 {
        logs.push(format!("INFO: Log entry {}", i));
    }
    let errors = LogProcessor::filter_errors(&logs);
    assert_eq!(errors.len(), 0);
}

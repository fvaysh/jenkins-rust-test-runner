pub struct LogProcessor;

impl LogProcessor {
    pub fn filter_errors(logs: &Vec<String>) -> Vec<String> {
        logs.iter()
            .filter(|line| line.contains("ERROR"))
            .cloned()
            .collect()
    }

    pub fn count_warnings(logs: &Vec<String>) -> usize {
        logs.iter().filter(|line| line.contains("WARNING")).count()
    }
}

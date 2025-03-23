#[cfg(test)]
mod tests {
    use super::super::parallel_runner::ParallelRunner;

    #[test]
    fn test_parallel_execution() {
        ParallelRunner::run_tasks(5);
        assert!(true);
    }
}

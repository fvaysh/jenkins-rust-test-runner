use std::time::Instant;
use super::super::parallel_runner::ParallelRunner;

#[test]
fn test_parallel_task_performance() {
    let start = Instant::now();
    ParallelRunner::run_tasks(1000);
    let duration = start.elapsed();
    assert!(duration.as_secs_f64() < 2.0, "Performance is too slow!");
}

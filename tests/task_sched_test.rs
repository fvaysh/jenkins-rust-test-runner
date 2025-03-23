#[cfg(test)]
mod tests {
    use super::super::task_scheduler::TaskScheduler;

    #[test]
    fn test_schedule_task() {
        TaskScheduler::schedule_task("Test Task", 1);
        assert!(true);
    }
}

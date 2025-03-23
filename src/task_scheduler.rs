use std::thread;
use std::time::Duration;

pub struct TaskScheduler;

impl TaskScheduler {
    pub fn schedule_task(task_name: &str, delay_secs: u64) {
        println!("Scheduling task: {} in {} seconds...", task_name, delay_secs);
        thread::sleep(Duration::from_secs(delay_secs));
        println!("Executing task: {}", task_name);
    }
}

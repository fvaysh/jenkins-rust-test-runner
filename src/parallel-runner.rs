use std::thread;

pub struct ParallelRunner;

impl ParallelRunner {
    pub fn run_tasks(task_count: usize) {
        let mut handles = vec![];
        for i in 0..task_count {
            handles.push(thread::spawn(move || {
                println!("Executing task {}", i);
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

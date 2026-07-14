use std::thread;
use std::time::Duration;

pub struct Scheduler;

impl Scheduler {
    pub fn spawn_interval<F>(job_name: &str, interval: Duration, mut task: F)
    where
        F: FnMut() + Send + 'static,
    {
        let job_name = job_name.to_string();

        println!("Scheduler started job: {}", job_name);

        thread::spawn(move || loop {
            thread::sleep(interval);

            println!("Scheduler running job: {}", job_name);

            task();
        });
    }
}
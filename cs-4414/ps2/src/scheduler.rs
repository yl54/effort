// This file will represent a simple scheduler.

// Use statements
use executor::Executor;
use thread;

// Struct for scheduler
pub struct Scheduler {
    
}

// scheduler implementation
impl Scheduler {
    // function to run an executor instance
    pub fn run_executor(&mut self, mut ex: Executor) -> thread::JoinHandle<String> {
        thread::spawn(move || {
            ex.run_cmd();
            println!("Gash Stuff");
            "SUCCESS!".to_string()
        })
    }
}

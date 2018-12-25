// This file will represent a simple scheduler.

// Use statements
use executor::Executor;
use std::fmt::{Display, Formatter, Result};
use thread;

// Struct for scheduler
pub struct Scheduler {
    // list of process struct
    process_list: Vec<Process>,

    // process count, for id
    process_count: u16,
}

// scheduler implementation
impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            process_list: vec![],
            process_count: 0,
        }
    }

    // function to run an executor instance
    pub fn run_executor(&mut self, mut ex: Executor) -> bool {
        let cmd_clone = ex.get_cmd();

        // Start the background process
        let h: thread::JoinHandle<String> = thread::spawn(move || {
            ex.run_cmd();
            "SUCCESS!".to_string()
        });

        // Record the process
        let p_info = ProcessInfo {
            id: self.process_count,
            cmd: cmd_clone,
        };
        let p = Process{
            p_info: p_info,
            handler: h,
        };
        self.process_list.push(p);
        
        // Update the process count for the next process
        self.process_count += 1;

        return true;
    }

    // function to get the process metadata. It excludes the handler
    fn get_process_list(&mut self) -> Vec<ProcessInfo> {
        let mut p_list: Vec<ProcessInfo> = vec![];

        for p in &self.process_list {
            let id = p.p_info.id;
            let cmd = p.p_info.cmd.clone();
            let p_info = ProcessInfo {
                id: id,
                cmd: cmd,
            };
            p_list.push(p_info);
        }
        
        return p_list;
    }

    // function to show a list of processes attached with the scheduler
    pub fn show_process_list(&mut self) {
        // Get the process list 
        let p_info_list = self.get_process_list();

        // Show the process list
        for p_info in &p_info_list {
            println!("{}", p_info);
        }
    }

    // function to kill a process
}

// Process represents a background process started from some external input
struct Process {
    p_info: ProcessInfo,
    handler: thread::JoinHandle<String>,
}

// ProcessInfo contains a processes metadata
struct ProcessInfo {
    id: u16,
    cmd: String,
}

impl Display for ProcessInfo {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}: {}", self.id, self.cmd)
    }
}

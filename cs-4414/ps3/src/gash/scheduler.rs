// This file will represent a base scheduler.

use std::fmt::{Display, Formatter, Result};
use std::thread;

use gash::executor::Executor;

// Schedulers schedules and records asynchronous processes that are running.
pub struct Scheduler {
    // process_list is a record of processes that have been launched by the scheduler.
    process_list: Vec<Process>,

    // process_count indicates how many processes are running at the moment.
    process_count: u16,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            process_list: vec![],
            process_count: 0,
        }
    }

    // run_executor runs a command asynchronously and records the process.
    pub fn run_executor(&mut self, mut ex: Executor) -> bool {
        let cmd_clone = ex.get_cmd();

        // Start the background process.
        let h: thread::JoinHandle<String> = thread::spawn(move || {
            ex.run_cmd();
            "SUCCESS!".to_string()
        });

        // Record the process in the scheduler.
        self.record_process(cmd_clone, h);

        return true;
    }

    // get_process_list gets a list of the processes and their metadata.
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

    // show_process_list shows a list of processes attached with the scheduler.
    pub fn show_process_list(&mut self) {
        // Get the process list 
        let p_info_list = self.get_process_list();

        // Show the process list
        for p_info in &p_info_list {
            debug!("{}", p_info);
        }
    }

    // record_process record and persits the process into the scheduler.
    fn record_process(&mut self, cmd: String, h: thread::JoinHandle<String>) {
        // Record the process
        let p_info = ProcessInfo {
            id: self.process_count,
            cmd: cmd,
        };
        let p = Process{
            p_info: p_info,
            handler: h,
        };

        // Add the process to the process list.
        self.process_list.push(p);
        
        // Increment the process count.
        self.process_count += 1;
    }
}

// Processes represents a background process started from some external input.
struct Process {
    p_info: ProcessInfo,
    handler: thread::JoinHandle<String>,
}

// ProcessInfo contains a processes metadata.
struct ProcessInfo {
    id: u16,
    cmd: String,
}

impl Display for ProcessInfo {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}: {}", self.id, self.cmd)
    }
}

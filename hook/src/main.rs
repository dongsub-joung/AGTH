use std::ffi::OsStr;

use sysinfo::{
    Components, Disks, Networks, Pid, Process, System
};

#[derive(Debug)]
struct Program{
    pid: Pid,
    process_name: OsStr,
}

impl Program {
    fn new(pid: Pid, process: Process) -> Program{
        let name= OsStr::new("null");
        
        // unknow size at compilation time 
        Program { pid, process_name: (*name) }
    }
}

fn main() {
    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    let mut target= String::new();
    for (pid, process) in sys.processes() {
        let process_name= process.name();
        println!("[{pid}] {:?} {:?}", process_name, process.disk_usage());
        let title= format!("{:?}", process_name);
        if target.contains(&title){
            // let program= Program::new(pid, process); Err

            
        }
    }
}
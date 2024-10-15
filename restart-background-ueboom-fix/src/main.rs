use sysinfo::{ProcessExt, System, SystemExt};
use std::{process::Command, time::Duration};

fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    let target_process = "background-ueboom-fix.exe";
    let replacement_exe = "C:/Users/Waradu/AppData/Local/Programs/WRDU/background-ueboom-fix.exe";
    for (_pid, process) in system.processes() {
        if process.name().to_lowercase() == target_process {
            println!("Killing process: {}", process.name());
            if process.kill() {
                println!("Process killed: {}", process.name());
            } else {
                println!("Failed to kill process: {}", process.name());
            }
        }
    }

    std::thread::sleep(Duration::from_secs(1));

    // Start the replacement executable
    match Command::new(replacement_exe).spawn() {
        Ok(_) => println!("Started new process: {}", replacement_exe),
        Err(e) => println!("Failed to start new process: {}", e),
    }
}

use chrono::Local;
use sysinfo::System;
use std::{thread, time::Duration};

fn display_ascii_rocket() {
    println!("     /\\");
    println!("    /  \\");
    println!("   /    \\");
    println!("  |------|");
    println!("  |      |");
    println!("  |      |");
    println!(" /|      |\\");
    println!("/ |      | \\");
    println!("  |------|");
    println!("  |      |");
    println!("  |      |");
    println!(" /|      |\\");
    println!("/ |      | \\");
    println!("  |------|");
    println!("  |  ||  |");
    println!("  \\__/\\__/");
    println!("     /\\");
    println!("    /  \\");
    println!("   /    \\");
}


fn display_system_info() {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    println!("System Information:");
    // Use static methods instead of instance methods based on error messages
    println!("OS: {}", System::name().unwrap_or_else(|| "Unknown".to_string()));
    println!("Kernel Version: {}", System::kernel_version().unwrap_or_else(|| "Unknown".to_string()));
    println!("CPU Count: {}", sys.cpus().len());
    println!("Total Memory: {} MB", sys.total_memory() / 1024);
    println!("Free Memory: {} MB", sys.free_memory() / 1024);
    println!("Used Memory: {} MB", (sys.total_memory() - sys.free_memory()) / 1024);
    
    // Skip disk information since disks() method is not available
    println!("Disk Usage: Not available in this sysinfo version");
    
    // Skip network information since networks() method is not available
    println!("Network Interfaces: Not available in this sysinfo version");
    
    //pid its give all processes
    /*println!("Processes:");
    for (pid, process) in sys.processes() {
        // Use Debug formatting for process name to handle OsStr
        println!(" PID: {}, Name: {:?}, Memory: {} MB", pid, process.name(), process.memory() / 1024 / 1024);
    }*/
    // Skip process information since processes() method is not available
    println!("Processes: Not available in this sysinfo version if you need it please uncomment the code");
    println!("-------------------------");
    
    
    println!("Uptime: {} seconds", System::uptime());
}

fn display_current_time() {
    println!("\nMission control center");
    println!("-------------------------");
    println!("Date: {}", Local::now().format("%Y-%m-%d"));
    println!("Time: {}", Local::now().format("%H:%M:%S"));
    println!("-------------------------");
    println!("Press Ctrl+C to exit.");
}

fn main() {
    println!("Welcome to the Mission Control Center!");
    display_ascii_rocket();
    display_current_time();
    
    // Simple loop without ctrlc dependency
    println!("\nRunning in continuous mode (press Ctrl+C to exit manually)");
    
    println!("-------------------------");
    println!("Mission Control Center");
    println!("-------------------------");
    display_system_info();
    println!("\nAll systems nominal. Mission control center is ready for launch.");
    println!("-------------------------");
    thread::sleep(Duration::from_secs(5));
    println!("\n");

    //keep fetching system info every 5 seconds
    /*loop {
        println!("-------------------------");
        println!("Mission Control Center");
        println!("-------------------------");
        display_system_info();
        println!("\nAll systems nominal. Mission control center is ready for launch.");
        println!("-------------------------");
        thread::sleep(Duration::from_secs(5));
        println!("\n");
    }*/
    // Uncomment the loop above to keep fetching system info every 5 seconds
    
}

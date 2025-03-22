use serde::Serialize;
use std::io;
use sysinfo::{Disks, System};

#[derive(Debug, Clone, Serialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub mem_used: f64,
    pub mem_total: f64,
    pub storage_used: u64,
    pub storage_total: u64,
}

pub fn get_system_metrics() -> io::Result<SystemMetrics> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let disks = Disks::new_with_refreshed_list();

    // CPU Usage (average across all cores)
    let cpu_usage = calculate_cpu_usage(&mut sys);

    // Memory (convert to GB)
    let (mem_used, mem_total) = calculate_memory_usage(&sys);

    // Storage
    let (storage_used, storage_total) = calculate_storage_usage(&disks);

    Ok(SystemMetrics {
        cpu_usage,
        mem_used,
        mem_total,
        storage_used,
        storage_total,
    })
}

// Calculates average CPU usage across all cores
fn calculate_cpu_usage(sys: &mut System) -> f32 {
    // First refresh to get initial CPU state
    sys.refresh_cpu_usage();

    // Wait a short time to measure CPU activity
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Second refresh to get updated CPU state
    sys.refresh_cpu_usage();

    let cpus = sys.cpus();
    if cpus.is_empty() {
        return 0.0;
    }

    cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32
}

// Calculates memory usage in GB
fn calculate_memory_usage(sys: &System) -> (f64, f64) {
    // Convert to GB
    let total = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used = (sys.total_memory() - sys.available_memory()) as f64 / 1024.0 / 1024.0 / 1024.0;

    (used, total)
}

// Calculates storage usage in bytes
fn calculate_storage_usage(disks: &Disks) -> (u64, u64) {
    let mut total: u64 = 0;
    let mut used: u64 = 0;

    for disk in disks {
        total += disk.total_space();
        used += disk.total_space() - disk.available_space();
    }

    (used, total)
}

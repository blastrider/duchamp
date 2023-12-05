extern crate chrono;
extern crate serde_json;
extern crate sys_info;

use chrono::prelude::*;
use std::process::Command;

fn get_sys_info() -> Result<(f64, f64, bool), Box<dyn std::error::Error>> {
    let mem_info = sys_info::mem_info()?;
    let disk_info = sys_info::disk_info()?;
    let ram_mb = convert_bytes_to_mb(mem_info.total - mem_info.avail);
    let disk_mb = convert_bytes_to_mb(disk_info.total - disk_info.free);
    let updates_available = check_updates();
    Ok((ram_mb, disk_mb, updates_available))
}

fn check_updates() -> bool {
    let output = Command::new("apt-get")
        .arg("-s") // simulate mode
        .arg("upgrade")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    !stdout.contains("0 upgraded, 0 newly installed, 0 to remove and 0 not upgraded")
}

// Convert bytes to megabytes
fn convert_bytes_to_mb(bytes: u64) -> f64 {
    (bytes as f64) / (1024.0 * 1024.0)
}

fn main() {
    match get_sys_info() {
        Ok((ram_mb, disk_mb, updates_available)) => {
            let now = Utc::now();
            let output = serde_json::json!({
                "date": now.to_rfc3339(),
                "ram_usage_MB": ram_mb,
                "disk_usage_MB": disk_mb,
                "updates_available": updates_available
            });
            println!("{}", output);
        }
        _ => println!("An error occurred"),
    }
}

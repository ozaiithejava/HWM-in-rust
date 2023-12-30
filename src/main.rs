use std::process::Command;
use std::str;

fn main() {
    // Ana kart kimliğini al
    let motherboard_id = get_hardware_id("Motherboard");

    // İşlemci kimliğini al
    let cpu_id = get_hardware_id("CPU");

    // Sabit disk kimliğini al
    let disk_id = get_hardware_id("Disk");

    // Ayrı ayrı alınan HWID'leri ekrana yazdır
    println!("Motherboard ID: {}", motherboard_id);
    println!("CPU ID: {}", cpu_id);
    println!("Disk ID: {}", disk_id);
}

fn get_hardware_id(hardware_type: &str) -> String {
    let os = std::env::consts::OS;

    // İşletim sistemine göre farklı komutlarla işlem yapılır
    match os {
        "windows" => {
            let output = Command::new("wmic")
                .args(&[hardware_type, "get", "SerialNumber"])
                .output()
                .expect("Failed to execute command");

            let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
            stdout.trim().to_string()
        }
        "linux" => {
            let command = match hardware_type {
                "CPU" => "cat /proc/cpuinfo | grep 'Serial' | awk '{print $3}'",
                "Disk" => "lsblk -no serial /dev/sda",
                _ => "",
            };

            let output = Command::new("sh")
                .args(&["-c", command])
                .output()
                .expect("Failed to execute command");

            let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
            stdout.trim().to_string()
        }
        _ => {
            panic!("Unsupported operating system");
        }
    }
}

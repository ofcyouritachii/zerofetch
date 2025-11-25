use super::Module;
use anyhow::Result;

pub struct BatteryModule {
    value: String,
}

impl BatteryModule {
    pub fn new() -> Result<Self> {
        let battery_info = Self::detect_battery();
        Ok(Self { value: battery_info })
    }

    fn detect_battery() -> String {
        #[cfg(target_os = "linux")]
        {
            use std::fs;
            
            // Try /sys/class/power_supply/BAT0 or BAT1
            for bat in &["BAT0", "BAT1", "battery"] {
                let base_path = format!("/sys/class/power_supply/{}", bat);
                let capacity_path = format!("{}/capacity", base_path);
                let status_path = format!("{}/status", base_path);
                
                if let (Ok(capacity), Ok(status)) = (
                    fs::read_to_string(&capacity_path),
                    fs::read_to_string(&status_path)
                ) {
                    return format!("{}% ({})", capacity.trim(), status.trim());
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            if let Ok(output) = Command::new("pmset").arg("-g").arg("batt").output() {
                if let Ok(out_str) = String::from_utf8(output.stdout) {
                    for line in out_str.lines() {
                        if line.contains('%') {
                            return line.trim().to_string();
                        }
                    }
                }
            }
        }

        "No battery".to_string()
    }
}

impl Module for BatteryModule {
    fn name(&self) -> &str {
        "Battery"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

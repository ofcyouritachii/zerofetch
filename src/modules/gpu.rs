use super::Module;
use anyhow::Result;
use std::process::Command;

pub struct GpuModule {
    value: String,
}

impl GpuModule {
    pub fn new() -> Result<Self> {
        let gpu = Self::detect_gpu();
        Ok(Self { value: gpu })
    }

    fn detect_gpu() -> String {
        #[cfg(target_os = "linux")]
        {
            // Try lspci first
            if let Ok(output) = Command::new("lspci").output() {
                if let Ok(out_str) = String::from_utf8(output.stdout) {
                    for line in out_str.lines() {
                        if line.contains("VGA") || line.contains("3D") {
                            if let Some(gpu_info) = line.split(':').nth(2) {
                                return gpu_info.trim().to_string();
                            }
                        }
                    }
                }
            }

            // Try /sys/class/drm
            if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
                for entry in entries.flatten() {
                    let path = entry.path().join("device/uevent");
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        for line in content.lines() {
                            if line.starts_with("PCI_ID=") {
                                return line.trim_start_matches("PCI_ID=").to_string();
                            }
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = Command::new("system_profiler")
                .arg("SPDisplaysDataType")
                .output()
            {
                if let Ok(out_str) = String::from_utf8(output.stdout) {
                    for line in out_str.lines() {
                        if line.contains("Chipset Model:") {
                            if let Some(model) = line.split(':').nth(1) {
                                return model.trim().to_string();
                            }
                        }
                    }
                }
            }
        }

        "Unknown".to_string()
    }
}

impl Module for GpuModule {
    fn name(&self) -> &str {
        "GPU"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

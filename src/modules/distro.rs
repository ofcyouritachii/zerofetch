use super::Module;
use anyhow::Result;
use std::fs;

pub struct DistroModule {
    value: String,
}

impl DistroModule {
    pub fn new() -> Result<Self> {
        let value = Self::detect_distro();
        Ok(Self { value })
    }

    fn detect_distro() -> String {
        #[cfg(target_os = "linux")]
        {
            // Try /etc/os-release first (modern standard)
            if let Ok(content) = fs::read_to_string("/etc/os-release") {
                for line in content.lines() {
                    if line.starts_with("PRETTY_NAME=") {
                        let name = line
                            .trim_start_matches("PRETTY_NAME=")
                            .trim_matches('"')
                            .to_string();
                        return name;
                    }
                }
            }

            // Fallback to lsb_release
            if let Ok(content) = fs::read_to_string("/etc/lsb-release") {
                for line in content.lines() {
                    if line.starts_with("DISTRIB_DESCRIPTION=") {
                        let name = line
                            .trim_start_matches("DISTRIB_DESCRIPTION=")
                            .trim_matches('"')
                            .to_string();
                        return name;
                    }
                }
            }

            "Linux".to_string()
        }
        #[cfg(target_os = "macos")]
        {
            Self::detect_macos_version()
        }
        #[cfg(target_os = "windows")]
        {
            Self::detect_windows_version()
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            "Unknown".to_string()
        }
    }

    #[cfg(target_os = "macos")]
    fn detect_macos_version() -> String {
        use std::process::Command;
        
        if let Ok(output) = Command::new("sw_vers").arg("-productVersion").output() {
            if let Ok(version) = String::from_utf8(output.stdout) {
                return format!("macOS {}", version.trim());
            }
        }
        "macOS".to_string()
    }

    #[cfg(target_os = "windows")]
    fn detect_windows_version() -> String {
        "Windows".to_string()
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    fn detect_macos_version() -> String {
        unreachable!()
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    fn detect_windows_version() -> String {
        unreachable!()
    }
}

impl Module for DistroModule {
    fn name(&self) -> &str {
        "Distro"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

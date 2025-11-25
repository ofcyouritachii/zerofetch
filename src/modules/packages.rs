use super::Module;
use anyhow::Result;
use std::process::Command;

pub struct PackagesModule {
    value: String,
}

impl PackagesModule {
    pub fn new() -> Result<Self> {
        let count = Self::count_packages();
        Ok(Self { value: count })
    }

    fn count_packages() -> String {
        let mut counts = Vec::new();

        #[cfg(target_os = "linux")]
        {
            // dpkg (Debian/Ubuntu)
            if let Some(count) = Self::count_dpkg() {
                counts.push(format!("{} (dpkg)", count));
            }

            // rpm (Fedora/RHEL)
            if let Some(count) = Self::count_rpm() {
                counts.push(format!("{} (rpm)", count));
            }

            // pacman (Arch)
            if let Some(count) = Self::count_pacman() {
                counts.push(format!("{} (pacman)", count));
            }

            // flatpak
            if let Some(count) = Self::count_flatpak() {
                counts.push(format!("{} (flatpak)", count));
            }

            // snap
            if let Some(count) = Self::count_snap() {
                counts.push(format!("{} (snap)", count));
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Some(count) = Self::count_brew() {
                counts.push(format!("{} (brew)", count));
            }
        }

        if counts.is_empty() {
            "Unknown".to_string()
        } else {
            counts.join(", ")
        }
    }

    #[cfg(target_os = "linux")]
    fn count_dpkg() -> Option<usize> {
        Command::new("dpkg-query")
            .args(["-f", ".", "-W"])
            .output()
            .ok()
            .map(|output| output.stdout.len())
    }

    #[cfg(target_os = "linux")]
    fn count_rpm() -> Option<usize> {
        Command::new("rpm")
            .arg("-qa")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|s| s.lines().count())
    }

    #[cfg(target_os = "linux")]
    fn count_pacman() -> Option<usize> {
        Command::new("pacman")
            .arg("-Qq")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|s| s.lines().count())
    }

    #[cfg(target_os = "linux")]
    fn count_flatpak() -> Option<usize> {
        Command::new("flatpak")
            .arg("list")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|s| s.lines().count())
    }

    #[cfg(target_os = "linux")]
    fn count_snap() -> Option<usize> {
        Command::new("snap")
            .arg("list")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|s| s.lines().count().saturating_sub(1)) // Subtract header line
    }

    #[cfg(target_os = "macos")]
    fn count_brew() -> Option<usize> {
        Command::new("brew")
            .arg("list")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|s| s.lines().count())
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    fn count_dpkg() -> Option<usize> { None }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    fn count_rpm() -> Option<usize> { None }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    fn count_pacman() -> Option<usize> { None }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    fn count_flatpak() -> Option<usize> { None }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    fn count_snap() -> Option<usize> { None }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    fn count_brew() -> Option<usize> { None }
}

impl Module for PackagesModule {
    fn name(&self) -> &str {
        "Packages"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

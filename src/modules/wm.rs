use super::Module;
use anyhow::Result;
use std::env;
use std::process::Command;

pub struct WindowManagerModule {
    value: String,
}

impl WindowManagerModule {
    pub fn new() -> Result<Self> {
        let wm = Self::detect_wm();
        Ok(Self { value: wm })
    }

    fn detect_wm() -> String {
        // Try environment variables first
        if let Ok(wm) = env::var("WAYLAND_DISPLAY") {
            if !wm.is_empty() {
                return Self::detect_wayland_compositor();
            }
        }

        if env::var("DISPLAY").is_ok() {
            return Self::detect_x11_wm();
        }

        "Unknown".to_string()
    }

    #[cfg(target_os = "linux")]
    fn detect_wayland_compositor() -> String {
        // Try to detect common Wayland compositors
        let compositors = ["sway", "hyprland", "wayfire", "river", "labwc"];
        
        for compositor in &compositors {
            if let Ok(output) = Command::new("pidof").arg(compositor).output() {
                if !output.stdout.is_empty() {
                    return compositor.to_string();
                }
            }
        }
        
        "Wayland".to_string()
    }

    #[cfg(target_os = "linux")]
    fn detect_x11_wm() -> String {
        // Try wmctrl first
        if let Ok(output) = Command::new("wmctrl").arg("-m").output() {
            if let Ok(out_str) = String::from_utf8(output.stdout) {
                for line in out_str.lines() {
                    if line.starts_with("Name:") {
                        return line.trim_start_matches("Name:").trim().to_string();
                    }
                }
            }
        }

        // Try xprop
        if let Ok(output) = Command::new("xprop")
            .args(["-root", "-notype", "_NET_WM_NAME"])
            .output()
        {
            if let Ok(out_str) = String::from_utf8(output.stdout) {
                if let Some(name) = out_str.split('=').nth(1) {
                    return name.trim().trim_matches('"').to_string();
                }
            }
        }

        "X11".to_string()
    }

    #[cfg(not(target_os = "linux"))]
    fn detect_wayland_compositor() -> String {
        "Unknown".to_string()
    }

    #[cfg(not(target_os = "linux"))]
    fn detect_x11_wm() -> String {
        "Unknown".to_string()
    }
}

impl Module for WindowManagerModule {
    fn name(&self) -> &str {
        "WM"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

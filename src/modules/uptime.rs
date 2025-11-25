use super::Module;
use anyhow::Result;
use sysinfo::System;

pub struct UptimeModule {
    value: String,
}

impl UptimeModule {
    pub fn new() -> Result<Self> {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        let uptime_secs = System::uptime();
        let value = Self::format_uptime(uptime_secs);
        
        Ok(Self { value })
    }

    fn format_uptime(seconds: u64) -> String {
        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        let minutes = (seconds % 3600) / 60;

        let mut parts = Vec::new();
        
        if days > 0 {
            parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
        }
        if hours > 0 {
            parts.push(format!("{} hour{}", hours, if hours == 1 { "" } else { "s" }));
        }
        if minutes > 0 || parts.is_empty() {
            parts.push(format!("{} min{}", minutes, if minutes == 1 { "" } else { "s" }));
        }

        parts.join(", ")
    }
}

impl Module for UptimeModule {
    fn name(&self) -> &str {
        "Uptime"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

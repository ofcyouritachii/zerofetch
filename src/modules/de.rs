use super::Module;
use anyhow::Result;
use std::env;

pub struct DesktopEnvironmentModule {
    value: String,
}

impl DesktopEnvironmentModule {
    pub fn new() -> Result<Self> {
        let de = env::var("XDG_CURRENT_DESKTOP")
            .ok()
            .or_else(|| env::var("DESKTOP_SESSION").ok())
            .or_else(|| env::var("GDMSESSION").ok())
            .unwrap_or_else(|| "Unknown".to_string());
        
        Ok(Self { value: de })
    }
}

impl Module for DesktopEnvironmentModule {
    fn name(&self) -> &str {
        "DE"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

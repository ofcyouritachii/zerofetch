use super::Module;
use anyhow::Result;

pub struct HostModule {
    value: String,
}

impl HostModule {
    pub fn new() -> Result<Self> {
        let hostname = hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "Unknown".to_string());
        
        let username = whoami::username();
        let value = format!("{}@{}", username, hostname);
        
        Ok(Self { value })
    }
}

impl Module for HostModule {
    fn name(&self) -> &str {
        "Host"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

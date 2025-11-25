use super::Module;
use anyhow::Result;

pub struct NetworkModule {
    value: String,
}

impl NetworkModule {
    pub fn new() -> Result<Self> {
        let local_ip = local_ip_address::local_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|_| "Unknown".to_string());
        
        Ok(Self { value: local_ip })
    }
}

impl Module for NetworkModule {
    fn name(&self) -> &str {
        "Local IP"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

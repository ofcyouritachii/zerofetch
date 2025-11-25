use super::Module;
use anyhow::Result;
use std::env;

pub struct TerminalModule {
    value: String,
}

impl TerminalModule {
    pub fn new() -> Result<Self> {
        let terminal = env::var("TERM")
            .ok()
            .or_else(|| env::var("TERM_PROGRAM").ok())
            .unwrap_or_else(|| "Unknown".to_string());
        
        Ok(Self { value: terminal })
    }
}

impl Module for TerminalModule {
    fn name(&self) -> &str {
        "Terminal"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

use super::Module;
use anyhow::Result;
use std::env;
use std::path::Path;

pub struct ShellModule {
    value: String,
}

impl ShellModule {
    pub fn new() -> Result<Self> {
        let shell = env::var("SHELL")
            .ok()
            .and_then(|s| {
                Path::new(&s)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.to_string())
            })
            .unwrap_or_else(|| "Unknown".to_string());
        
        let value = shell;
        Ok(Self { value })
    }
}

impl Module for ShellModule {
    fn name(&self) -> &str {
        "Shell"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

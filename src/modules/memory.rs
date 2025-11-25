use super::Module;
use anyhow::Result;
use sysinfo::System;

pub struct MemoryModule {
    value: String,
}

impl MemoryModule {
    pub fn new() -> Result<Self> {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        let total_mem = sys.total_memory() / 1024 / 1024; // Convert to MB
        let used_mem = sys.used_memory() / 1024 / 1024;
        
        let value = format!("{} MiB / {} MiB", used_mem, total_mem);
        
        Ok(Self { value })
    }
}

impl Module for MemoryModule {
    fn name(&self) -> &str {
        "Memory"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

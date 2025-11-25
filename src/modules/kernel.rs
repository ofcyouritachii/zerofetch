use super::Module;
use anyhow::Result;
use sysinfo::System;

pub struct KernelModule {
    value: String,
}

impl KernelModule {
    pub fn new() -> Result<Self> {
        let value = System::kernel_version()
            .unwrap_or_else(|| "Unknown".to_string());
        
        Ok(Self { value })
    }
}

impl Module for KernelModule {
    fn name(&self) -> &str {
        "Kernel"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

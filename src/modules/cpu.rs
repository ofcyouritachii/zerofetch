use super::Module;
use anyhow::Result;
use sysinfo::System;

pub struct CpuModule {
    value: String,
}

impl CpuModule {
    pub fn new() -> Result<Self> {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        let cpu_info = if let Some(cpu) = sys.cpus().first() {
            let brand = cpu.brand();
            let cores = sys.cpus().len();
            format!("{} ({} cores)", brand, cores)
        } else {
            "Unknown".to_string()
        };
        
        Ok(Self { value: cpu_info })
    }
}

impl Module for CpuModule {
    fn name(&self) -> &str {
        "CPU"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

use super::Module;
use anyhow::Result;
use sysinfo::Disks;

pub struct DiskModule {
    value: String,
}

impl DiskModule {
    pub fn new() -> Result<Self> {
        let disks = Disks::new_with_refreshed_list();
        
        let mut total_space = 0u64;
        let mut available_space = 0u64;
        
        for disk in disks.list() {
            total_space += disk.total_space();
            available_space += disk.available_space();
        }
        
        let used_space = total_space - available_space;
        
        let total_gb = total_space / 1024 / 1024 / 1024;
        let used_gb = used_space / 1024 / 1024 / 1024;
        
        let value = format!("{} GiB / {} GiB", used_gb, total_gb);
        
        Ok(Self { value })
    }
}

impl Module for DiskModule {
    fn name(&self) -> &str {
        "Disk"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

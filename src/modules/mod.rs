pub mod cpu;
pub mod disk;
pub mod distro;
pub mod gpu;
pub mod host;
pub mod kernel;
pub mod memory;
pub mod network;
pub mod os;
pub mod packages;
pub mod shell;
pub mod terminal;
pub mod uptime;
pub mod de;
pub mod wm;
pub mod battery;

use crate::config::Config;
use anyhow::Result;
use std::collections::HashMap;

pub trait Module: Send + Sync {
    fn name(&self) -> &str;
    fn value(&self) -> Result<String>;
    fn enabled(&self, config: &Config) -> bool {
        !config.modules.disabled.contains(&self.name().to_string())
    }
}

pub struct SystemInfo {
    modules: HashMap<String, Box<dyn Module>>,
}

impl SystemInfo {
    pub fn new(config: &Config) -> Result<Self> {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

        // Register all available modules
        modules.insert("os".to_string(), Box::new(os::OsModule::new()?));
        modules.insert("host".to_string(), Box::new(host::HostModule::new()?));
        modules.insert("kernel".to_string(), Box::new(kernel::KernelModule::new()?));
        modules.insert("uptime".to_string(), Box::new(uptime::UptimeModule::new()?));
        modules.insert("packages".to_string(), Box::new(packages::PackagesModule::new()?));
        modules.insert("shell".to_string(), Box::new(shell::ShellModule::new()?));
        modules.insert("de".to_string(), Box::new(de::DesktopEnvironmentModule::new()?));
        modules.insert("wm".to_string(), Box::new(wm::WindowManagerModule::new()?));
        modules.insert("terminal".to_string(), Box::new(terminal::TerminalModule::new()?));
        modules.insert("cpu".to_string(), Box::new(cpu::CpuModule::new()?));
        modules.insert("gpu".to_string(), Box::new(gpu::GpuModule::new()?));
        modules.insert("memory".to_string(), Box::new(memory::MemoryModule::new()?));
        modules.insert("disk".to_string(), Box::new(disk::DiskModule::new()?));
        modules.insert("network".to_string(), Box::new(network::NetworkModule::new()?));
        modules.insert("battery".to_string(), Box::new(battery::BatteryModule::new()?));
        modules.insert("distro".to_string(), Box::new(distro::DistroModule::new()?));

        Ok(Self { modules })
    }

    pub fn get_module(&self, name: &str) -> Option<&Box<dyn Module>> {
        self.modules.get(name)
    }

    pub fn get_ordered_modules(&self, config: &Config) -> Vec<&Box<dyn Module>> {
        let mut result = Vec::new();
        
        for module_name in &config.modules.order {
            if let Some(module) = self.modules.get(module_name) {
                if module.enabled(config) {
                    result.push(module);
                }
            }
        }
        
        result
    }

    pub fn all_modules(&self) -> &HashMap<String, Box<dyn Module>> {
        &self.modules
    }
}

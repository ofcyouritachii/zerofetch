use super::Module;
use anyhow::Result;

pub struct OsModule {
    value: String,
}

impl OsModule {
    pub fn new() -> Result<Self> {
        let value = Self::detect_os();
        Ok(Self { value })
    }

    fn detect_os() -> String {
        #[cfg(target_os = "linux")]
        {
            "Linux".to_string()
        }
        #[cfg(target_os = "macos")]
        {
            "macOS".to_string()
        }
        #[cfg(target_os = "windows")]
        {
            "Windows".to_string()
        }
        #[cfg(target_os = "freebsd")]
        {
            "FreeBSD".to_string()
        }
        #[cfg(target_os = "openbsd")]
        {
            "OpenBSD".to_string()
        }
        #[cfg(target_os = "netbsd")]
        {
            "NetBSD".to_string()
        }
        #[cfg(target_os = "dragonfly")]
        {
            "DragonFly BSD".to_string()
        }
        #[cfg(target_os = "android")]
        {
            "Android".to_string()
        }
        #[cfg(not(any(
            target_os = "linux",
            target_os = "macos",
            target_os = "windows",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "dragonfly",
            target_os = "android"
        )))]
        {
            "Unknown".to_string()
        }
    }
}

impl Module for OsModule {
    fn name(&self) -> &str {
        "OS"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}

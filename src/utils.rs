use std::fs;

pub fn detect_distro() -> String {
    #[cfg(target_os = "linux")]
    {
        // Try /etc/os-release
        if let Ok(content) = fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if line.starts_with("ID=") {
                    return line
                        .trim_start_matches("ID=")
                        .trim_matches('"')
                        .to_string();
                }
            }
        }
        
        "linux".to_string()
    }
    
    #[cfg(target_os = "macos")]
    {
        "macos".to_string()
    }
    
    #[cfg(target_os = "windows")]
    {
        "windows".to_string()
    }
    
    #[cfg(target_os = "freebsd")]
    {
        "freebsd".to_string()
    }
    
    #[cfg(target_os = "openbsd")]
    {
        "openbsd".to_string()
    }
    
    #[cfg(target_os = "android")]
    {
        "android".to_string()
    }
    
    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "windows",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "android"
    )))]
    {
        "unknown".to_string()
    }
}

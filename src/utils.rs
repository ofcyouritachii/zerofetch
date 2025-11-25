use std::fs;

/// Detect Linux distribution from /etc/os-release
pub fn detect_distro() -> String {
    // Read /etc/os-release for distribution ID
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
    
    // Fallback to generic linux
    "linux".to_string()
}

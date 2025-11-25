use crate::config::Config;
use crate::logo::LogoEngine;
use crate::modules::SystemInfo;
use anyhow::Result;
use colored::*;

pub fn render(system_info: &SystemInfo, config: &Config) -> Result<()> {
    // Handle special output formats
    match config.general.format.as_str() {
        "json" => return render_json(system_info, config),
        "yaml" => return render_yaml(system_info, config),
        "toml" => return render_toml(system_info, config),
        "plain" => return render_plain(system_info, config),
        _ => {}
    }

    // Default rendering
    let logo_engine = LogoEngine::new(config)?;
    let logo_lines = logo_engine.get_lines();
    let modules = system_info.get_ordered_modules(config);

    // Get module info lines
    let mut info_lines = Vec::new();
    for module in modules {
        if let Ok(value) = module.value() {
            let line = format!(
                "{}{} {}",
                module.name().bright_blue().bold(),
                config.general.separator.bright_black(),
                value.white()
            );
            info_lines.push(line);
        }
    }

    // Combine logo and info side by side
    let logo_width = logo_lines
        .iter()
        .map(|l| strip_ansi_codes(l).len())
        .max()
        .unwrap_or(0);
    
    let padding = " ".repeat(config.logo.padding);
    let max_lines = logo_lines.len().max(info_lines.len());

    for i in 0..max_lines {
        let logo_line = if i < logo_lines.len() {
            &logo_lines[i]
        } else {
            ""
        };

        let info_line = if i < info_lines.len() {
            &info_lines[i]
        } else {
            ""
        };

        // Calculate actual display width (without ANSI codes)
        let logo_display_len = strip_ansi_codes(logo_line).len();
        let space_needed = if logo_display_len < logo_width {
            logo_width - logo_display_len
        } else {
            0
        };

        println!(
            "{}{}{}{}",
            logo_line,
            " ".repeat(space_needed),
            padding,
            info_line
        );
    }

    Ok(())
}

fn render_json(system_info: &SystemInfo, config: &Config) -> Result<()> {
    use std::collections::HashMap;
    
    let modules = system_info.get_ordered_modules(config);
    let mut data = HashMap::new();
    
    for module in modules {
        if let Ok(value) = module.value() {
            data.insert(module.name().to_string(), value);
        }
    }
    
    println!("{}", serde_json::to_string_pretty(&data)?);
    Ok(())
}

fn render_yaml(system_info: &SystemInfo, config: &Config) -> Result<()> {
    use std::collections::HashMap;
    
    let modules = system_info.get_ordered_modules(config);
    let mut data = HashMap::new();
    
    for module in modules {
        if let Ok(value) = module.value() {
            data.insert(module.name().to_string(), value);
        }
    }
    
    println!("{}", serde_yaml::to_string(&data)?);
    Ok(())
}

fn render_toml(system_info: &SystemInfo, config: &Config) -> Result<()> {
    use std::collections::HashMap;
    
    let modules = system_info.get_ordered_modules(config);
    let mut data = HashMap::new();
    
    for module in modules {
        if let Ok(value) = module.value() {
            data.insert(module.name().to_string(), value);
        }
    }
    
    println!("{}", toml::to_string(&data)?);
    Ok(())
}

fn render_plain(system_info: &SystemInfo, config: &Config) -> Result<()> {
    let modules = system_info.get_ordered_modules(config);
    
    for module in modules {
        if let Ok(value) = module.value() {
            println!("{}{} {}", module.name(), config.general.separator, value);
        }
    }
    
    Ok(())
}

fn strip_ansi_codes(s: &str) -> String {
    // Simple ANSI code stripper for width calculation
    let mut result = String::new();
    let mut in_escape = false;
    
    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else {
            result.push(c);
        }
    }
    
    result
}

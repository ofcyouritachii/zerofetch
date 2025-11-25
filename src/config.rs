use crate::cli::{Args, LogoType, OutputFormat};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    
    #[serde(default)]
    pub logo: LogoConfig,
    
    #[serde(default)]
    pub display: DisplayConfig,
    
    #[serde(default)]
    pub modules: ModulesConfig,
    
    #[serde(default)]
    pub colors: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_separator")]
    pub separator: String,
    
    #[serde(default)]
    pub compact: bool,
    
    #[serde(default = "default_format")]
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoConfig {
    #[serde(default = "default_logo_type")]
    pub logo_type: String,
    
    #[serde(default)]
    pub logo_name: Option<String>,
    
    #[serde(default)]
    pub logo_file: Option<String>,
    
    #[serde(default)]
    pub logo_url: Option<String>,
    
    #[serde(default = "default_padding")]
    pub padding: usize,
    
    #[serde(default)]
    pub width: Option<u32>,
    
    #[serde(default)]
    pub height: Option<u32>,
    
    #[serde(default)]
    pub color_override: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    #[serde(default)]
    pub show_colors: bool,
    
    #[serde(default = "default_color_scheme")]
    pub color_scheme: String,
    
    #[serde(default)]
    pub no_color: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModulesConfig {
    #[serde(default = "default_module_order")]
    pub order: Vec<String>,
    
    #[serde(default)]
    pub enabled: Vec<String>,
    
    #[serde(default)]
    pub disabled: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            logo: LogoConfig::default(),
            display: DisplayConfig::default(),
            modules: ModulesConfig::default(),
            colors: default_colors(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            separator: default_separator(),
            compact: false,
            format: default_format(),
        }
    }
}

impl Default for LogoConfig {
    fn default() -> Self {
        Self {
            logo_type: default_logo_type(),
            logo_name: None,
            logo_file: None,
            logo_url: None,
            padding: default_padding(),
            width: None,
            height: None,
            color_override: None,
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_colors: false,
            color_scheme: default_color_scheme(),
            no_color: false,
        }
    }
}

impl Default for ModulesConfig {
    fn default() -> Self {
        Self {
            order: default_module_order(),
            enabled: vec![],
            disabled: vec![],
        }
    }
}

fn default_separator() -> String {
    ":".to_string()
}

fn default_format() -> String {
    "default".to_string()
}

fn default_logo_type() -> String {
    "auto".to_string()
}

fn default_padding() -> usize {
    2
}

fn default_color_scheme() -> String {
    "default".to_string()
}

fn default_module_order() -> Vec<String> {
    vec![
        "os".to_string(),
        "host".to_string(),
        "kernel".to_string(),
        "uptime".to_string(),
        "packages".to_string(),
        "shell".to_string(),
        "de".to_string(),
        "wm".to_string(),
        "terminal".to_string(),
        "cpu".to_string(),
        "gpu".to_string(),
        "memory".to_string(),
        "disk".to_string(),
    ]
}

fn default_colors() -> HashMap<String, String> {
    let mut colors = HashMap::new();
    colors.insert("title".to_string(), "cyan".to_string());
    colors.insert("key".to_string(), "blue".to_string());
    colors.insert("value".to_string(), "white".to_string());
    colors.insert("separator".to_string(), "gray".to_string());
    colors
}

impl Config {
    /// Load configuration with priority: CLI > User Config > System Config > Defaults
    pub fn load(args: &Args) -> Result<Self> {
        let mut config = Config::default();

        // Try loading system config
        if let Some(system_path) = Self::system_config_path() {
            if system_path.exists() {
                if let Ok(sys_config) = Self::load_from_file(&system_path) {
                    config = Self::merge(config, sys_config);
                }
            }
        }

        // Try loading user config
        if let Some(user_path) = Self::user_config_path() {
            if user_path.exists() {
                config = Self::load_from_file(&user_path)
                    .context("Failed to load user config")?;
            }
        }

        // Load custom config if specified
        if let Some(custom_path) = &args.config {
            config = Self::load_from_file(PathBuf::from(custom_path).as_path())
                .context("Failed to load custom config")?;
        }

        // Apply CLI overrides
        config.apply_cli_overrides(args);

        Ok(config)
    }

    fn load_from_file(path: &std::path::Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        
        let config = if path.extension().and_then(|s| s.to_str()) == Some("yaml") 
            || path.extension().and_then(|s| s.to_str()) == Some("yml") {
            serde_yaml::from_str(&content)?
        } else if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            toml::from_str(&content)?
        } else {
            // Default to JSON/JSONC
            json5::from_str(&content)?
        };
        
        Ok(config)
    }

    fn user_config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|mut path| {
            path.push("zerofetch");
            path.push("config.jsonc");
            path
        })
    }

    fn system_config_path() -> Option<PathBuf> {
        #[cfg(unix)]
        {
            Some(PathBuf::from("/etc/zerofetch/config.jsonc"))
        }
        #[cfg(windows)]
        {
            None
        }
    }

    fn merge(mut base: Config, override_config: Config) -> Config {
        // Simple merge - override config takes precedence
        if !override_config.colors.is_empty() {
            base.colors = override_config.colors;
        }
        base.general = override_config.general;
        base.logo = override_config.logo;
        base.display = override_config.display;
        base.modules = override_config.modules;
        base
    }

    fn apply_cli_overrides(&mut self, args: &Args) {
        if !args.separator.is_empty() {
            self.general.separator = args.separator.clone();
        }
        
        if args.compact {
            self.general.compact = true;
        }
        
        if args.no_color {
            self.display.no_color = true;
        }
        
        if let Some(color) = &args.color {
            self.display.color_scheme = color.clone();
        }
        
        if let Some(logo) = &args.logo {
            self.logo.logo_name = Some(logo.clone());
        }
        
        if let Some(logo_type) = &args.logo_type {
            self.logo.logo_type = format!("{:?}", logo_type).to_lowercase();
        }
        
        if let Some(padding) = args.logo_padding {
            self.logo.padding = padding;
        }
        
        if let Some(file) = &args.logo_file {
            self.logo.logo_file = Some(file.clone());
        }
        
        if let Some(url) = &args.logo_url {
            self.logo.logo_url = Some(url.clone());
        }
        
        if let Some(width) = args.logo_width {
            self.logo.width = Some(width);
        }
        
        if let Some(height) = args.logo_height {
            self.logo.height = Some(height);
        }
        
        if let Some(modules) = &args.modules {
            self.modules.enabled = modules.clone();
        }
        
        if let Some(hide) = &args.hide {
            self.modules.disabled = hide.clone();
        }
    }

    pub fn generate_default_config_file() -> Result<String> {
        let config = Config::default();
        let json = serde_json::to_string_pretty(&config)?;
        Ok(json)
    }
}

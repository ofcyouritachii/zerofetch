pub mod ascii;

use crate::config::Config;
use anyhow::Result;

pub enum Logo {
    Ascii(Vec<String>),
    None,
}

pub struct LogoEngine {
    logo: Logo,
}

impl LogoEngine {
    pub fn new(config: &Config) -> Result<Self> {
        let logo = match config.logo.logo_type.as_str() {
            "none" => Logo::None,
            "ascii" | "auto" | _ => {
                let distro = crate::utils::detect_distro();
                let logo_lines = ascii::get_logo(&distro);
                Logo::Ascii(logo_lines)
            }
        };

        Ok(Self { logo })
    }

    pub fn get_lines(&self) -> Vec<String> {
        match &self.logo {
            Logo::Ascii(lines) => lines.clone(),
            Logo::None => vec![],
        }
    }

    pub fn height(&self) -> usize {
        match &self.logo {
            Logo::Ascii(lines) => lines.len(),
            Logo::None => 0,
        }
    }
}

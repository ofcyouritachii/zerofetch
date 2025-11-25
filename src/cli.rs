use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "zerofetch",
    version,
    about = "A high-performance, modular, cross-platform system information CLI tool",
    long_about = "ZeroFetch - An advanced system information tool inspired by Fastfetch and Neofetch.\n\
                  Displays system information with beautiful ASCII art logos and extensive customization."
)]
pub struct Args {
    /// Path to custom configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<String>,

    /// Logo to display (auto, ascii, kitty, iterm, none, or distro name)
    #[arg(short, long)]
    pub logo: Option<String>,

    /// Logo type override
    #[arg(long, value_enum)]
    pub logo_type: Option<LogoType>,

    /// Color scheme (0-255 for 8-bit, or color name)
    #[arg(long)]
    pub color: Option<String>,

    /// Disable all colors
    #[arg(long)]
    pub no_color: bool,

    /// Separator character between key and value
    #[arg(short, long, default_value = ":")]
    pub separator: String,

    /// Show only specific modules (comma-separated)
    #[arg(short, long, value_delimiter = ',')]
    pub modules: Option<Vec<String>>,

    /// Hide specific modules (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub hide: Option<Vec<String>>,

    /// Output format
    #[arg(short, long, value_enum, default_value = "default")]
    pub format: OutputFormat,

    /// Compact mode (minimal spacing)
    #[arg(long)]
    pub compact: bool,

    /// Logo padding (number of spaces)
    #[arg(long)]
    pub logo_padding: Option<usize>,

    /// Custom logo file path
    #[arg(long)]
    pub logo_file: Option<String>,

    /// Remote logo URL
    #[arg(long)]
    pub logo_url: Option<String>,

    /// Logo width (for image scaling)
    #[arg(long)]
    pub logo_width: Option<u32>,

    /// Logo height (for image scaling)
    #[arg(long)]
    pub logo_height: Option<u32>,

    /// List all available modules
    #[arg(long)]
    pub list_modules: bool,

    /// List all available logos
    #[arg(long)]
    pub list_logos: bool,

    /// Generate default config file
    #[arg(long)]
    pub gen_config: bool,

    /// Benchmark mode (test syscall speed)
    #[arg(long)]
    pub benchmark: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum LogoType {
    Auto,
    Ascii,
    Kitty,
    Iterm,
    None,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Default,
    Json,
    Yaml,
    Toml,
    Plain,
}

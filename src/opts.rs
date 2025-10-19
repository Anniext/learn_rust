use clap::Parser;
use std::fmt::{self};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[command(name="rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand, // 子命令参数
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    Csv(CsvOpts), // csv参数
}

#[derive(Debug, Clone, Copy)]
pub enum OutFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
// CsvOpts csv参数
pub struct CsvOpts {
    #[arg(short, long, value_parser=verify_file_exists)]
    pub input: String, // 输入文件路径
    #[arg(short, long)]
    pub output: Option<String>, // 输出文件路径
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char, // 分隔符
    #[arg(long, default_value_t = true)]
    pub header: bool, // 是否有表头
    #[arg(long, value_parser = parse_output_format, default_value = "json")]
    pub format: OutFormat, // 是否有表头
}

// verify_file_exists 验证文件是否存在
fn verify_file_exists(file_name: &str) -> Result<String, &'static str> {
    if Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File does not exist")
    }
}

// parse_output_format 解析输出格式
fn parse_output_format(format: &str) -> Result<OutFormat, anyhow::Error> {
    format.parse::<OutFormat>()
}

// FromStr 实现 OutFormat 的 FromStr trait
impl FromStr for OutFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<OutFormat, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutFormat::Json),
            "yaml" => Ok(OutFormat::Yaml),
            "toml" => Ok(OutFormat::Toml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

// From 实现 OutFormat 的 From trait
impl From<OutFormat> for &'static str {
    fn from(format: OutFormat) -> Self {
        match format {
            OutFormat::Json => "json",
            OutFormat::Yaml => "yaml",
            OutFormat::Toml => "toml",
        }
    }
}

// Display 实现 OutFormat 的 Display trait
impl fmt::Display for OutFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

use clap::Parser;
use std::path::Path;

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

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser=verify_file_exists)]
    pub input: String, // 输入文件路径
    #[arg(short, long, default_value = "output.json")]
    pub output: String, // 输出文件路径
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char, // 分隔符
    #[arg(long, default_value_t = true)]
    pub header: bool, // 是否有表头
}

fn verify_file_exists(file_name: &str) -> Result<String, &'static str> {
    if Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File does not exist")
    }
}

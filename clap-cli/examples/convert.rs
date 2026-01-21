use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "converter", version = "0.0.1", about = "文件格式转换工具")]
struct Args {
    /// 输入文件
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// 输出文件
    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,

    /// 输出格式
    #[arg(short, long, value_enum, default_value = "json")]
    format: Format,
}

#[derive(Clone, Debug, clap::ValueEnum)]
enum Format {
    Json,
    Yaml,
    Toml,
}

fn main() {
    let args = Args::parse();
    println!(
        "转换 {:?} -> {:?} (格式: {:?})",
        args.input, args.output, args.format
    );
}

use clap::Parser;

/// 简单的文件搜索工具
#[derive(Parser, Debug)]
#[command(name = "mysearch")]
#[command(about = "搜索文件中的模式", long_about = None)]
struct Args {
    /// 搜索模式
    #[arg(short, long)]
    pattern: String,

    /// 搜索目录
    #[arg(short, long, default_value = ".")]
    dir: String,

    /// 详细输出
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    println!("搜索模式: {}", args.pattern);
    println!("搜索目录: {}", args.dir);
    println!("详细模式: {}", args.verbose);
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "simple-git")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 添加文件
    Add {
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// 提交更改
    Commit {
        #[arg(short, long)]
        message: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { files } => {
            println!("添加文件: {:?}", files);
        }
        Commands::Commit { message } => {
            println!("提交信息: {}", message);
        }
    }
}

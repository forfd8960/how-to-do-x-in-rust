use clap::Parser;

fn validate_port(s: &str) -> Result<u16, String> {
    let port: u16 = s.parse().map_err(|_| format!("端口必须是数字"))?;

    if port < 1024 {
        return Err(format!("端口必须 >= 1024（当前: {}）", port));
    }
    Ok(port)
}

#[derive(Parser)]
struct Args {
    /// 服务器端口
    #[arg(long, value_parser = validate_port)]
    port: u16,

    /// 最大连接数（1-1000）
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..=1000))]
    max_connections: u32,
}

fn main() {
    let args = Args::parse();
    println!("端口: {}, 最大连接: {}", args.port, args.max_connections);
}

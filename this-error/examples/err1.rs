use thiserror::Error;

// 用 derive 宏自动生成错误实现
#[derive(Error, Debug)]
pub enum DataError {
    #[error("文件: {0} 未找到")]
    FileNotFound(String),

    #[error("解析失败在第 {line} 行")]
    ParseError { line: usize },

    #[error("IO 错误")]
    Io(#[from] std::io::Error), // 自动实现 From 转换
}

fn main() {
    let err = DataError::FileNotFound("data.txt".to_string());
    println!("{}", err); // 输出: 文件未找到: data.txt
}

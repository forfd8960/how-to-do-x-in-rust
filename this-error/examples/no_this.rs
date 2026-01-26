use std::fmt;

#[derive(Debug)]
pub enum MyError {
    IoError(std::io::Error),
    ParseError(String),
}

// 手写 Display
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::IoError(e) => write!(f, "IO error: {}", e),
            MyError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

// 手写 Error trait
impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::IoError(e) => Some(e),
            MyError::ParseError(_) => None,
        }
    }
}

// 还要手写 From 转换...
impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::IoError(err)
    }
}

fn main() {
    let err = MyError::ParseError("parse failed".to_string());
    println!("{}", err); // 输出: 解析错误: parse failed
}
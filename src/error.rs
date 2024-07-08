pub type Result<T, E = SimpleError> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum SimpleError {
    Parser(String),
    Lexer(String),
    Impl(String),
}

impl std::fmt::Display for SimpleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpleError::Parser(msg) => write!(f, "Parser error: {msg}"),
            SimpleError::Lexer(msg) => write!(f, "Lexer error: {msg}"),
            SimpleError::Impl(msg) => write!(f, "Impl error: {msg}"),
        }
    }
}

impl std::error::Error for SimpleError {}

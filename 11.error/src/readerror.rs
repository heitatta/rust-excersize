use std::fmt;

#[derive(Debug)]
pub enum ReadError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Negative(i32),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadError::Io(e) => write!(f, "IO error: {}", e),
            ReadError::Parse(e) => write!(f, "Parse error: {}", e),
            ReadError::Negative(n) => write!(f, "Negative number not accepted: {}", n),
        }
    }
}

impl std::error::Error for ReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ReadError::Io(e) => Some(e),
            ReadError::Parse(e) => Some(e),
            ReadError::Negative(_) => None,
        }
    }
}

impl From<std::io::Error> for ReadError {
    fn from(err: std::io::Error) -> ReadError {
        ReadError::Io(err)
    }
}

impl From<std::num::ParseIntError> for ReadError {
    fn from(err: std::num::ParseIntError) -> ReadError {
        ReadError::Parse(err)
    }
}

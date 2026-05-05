#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Custom(String),
    InvalidValue,
    OutOfRange,
    TrailingBytes,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Custom(msg) => write!(f, "Custom error: {}", msg),
            Error::InvalidValue => write!(f, "Invalid value"),
            Error::OutOfRange => write!(f, "Out of range"),
            Error::TrailingBytes => write!(f, "Trailing bytes"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Custom(_) => None,
            Error::InvalidValue => None,
            Error::OutOfRange => None,
            Error::TrailingBytes => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

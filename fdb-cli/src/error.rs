use std::fmt::Formatter;
use foundationdb::FdbError;

#[derive(Debug)]
pub enum Error {
    UnsupportedOperatingSystem(&'static str),
    UnableToReadConfig(std::io::Error),
    UnableToWriteConfig(std::io::Error),
    Fdb(FdbError),
    Elapsed(tokio::time::error::Elapsed)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::UnsupportedOperatingSystem(os) => write!(f, "{}", os),
            Error::UnableToReadConfig(ref err) => write!(f, "{}", err),
            Error::UnableToWriteConfig(ref err) => write!(f, "{}", err),
            Error::Fdb(ref e) => write!(f, "Fdb error: {}", e),
            Error::Elapsed(ref e) => write!(f, "Tokio timeout elapsed error: {}", e)
        }
    }
}

impl From<FdbError> for Error {
    fn from(err: FdbError) -> Error {
        Error::Fdb(err)
    }
}

impl From<tokio::time::error::Elapsed> for Error {
    fn from(err: tokio::time::error::Elapsed) -> Error {
        Error::Elapsed(err)
    }
}
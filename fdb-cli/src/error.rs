use std::fmt::Formatter;

#[derive(Debug)]
pub enum Error {
    UnsupportedOperatingSystem(&'static str),
    UnableToReadConfig(std::io::Error),
    UnableToWriteConfig(std::io::Error)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::UnsupportedOperatingSystem(os) => write!(f, "{}", os),
            Error::UnableToReadConfig(ref err) => write!(f, "{}", err),
            Error::UnableToWriteConfig(ref err) => write!(f, "{}", err)
        }
    }
}
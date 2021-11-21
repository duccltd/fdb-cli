use foundationdb::FdbError;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum Error {
    UnsupportedOperatingSystem(&'static str),
    UnableToReadConfig(std::io::Error),
    UnableToWriteConfig(std::io::Error),
    UnableToReadProtobuf(std::io::Error),
    Fdb(FdbError),
    Elapsed(tokio::time::error::Elapsed),
    ProtofishParseError(protofish::context::ParseError),
    StringDecodeError(std::string::FromUtf8Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::UnsupportedOperatingSystem(os) => write!(f, "Unsupported OS: {}", os),
            Error::UnableToReadConfig(ref err) => {
                write!(f, "Unable to read configuration: {}", err)
            }
            Error::UnableToWriteConfig(ref err) => {
                write!(f, "Unable to write configuration: {}", err)
            }
            Error::UnableToReadProtobuf(ref err) => write!(f, "Unable to read protobuf: {}", err),
            Error::Fdb(ref e) => write!(f, "Fdb error: {}", e),
            Error::Elapsed(ref e) => write!(f, "Tokio timeout elapsed error: {}", e),
            Error::ProtofishParseError(ref e) => write!(f, "Protofish parse error: {}", e),
            Error::StringDecodeError(ref e) => write!(f, "String decode error: {}", e),
        }
    }
}

impl From<trompt::Error> for Error {
    fn from(_: trompt::Error) -> Self {
        todo!()
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

impl From<protofish::context::ParseError> for Error {
    fn from(err: protofish::context::ParseError) -> Error {
        Error::ProtofishParseError(err)
    }
}

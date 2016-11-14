use std::io;
use record::ParseSfvRecordError;

pub enum Error {
    IO(io::Error),
    Format(ParseSfvRecordError),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<ParseSfvRecordError> for Error {
    fn from(error: ParseSfvRecordError) -> Self {
        Error::Format(error)
    }
}

use crc32::Crc32Digest;
use std::io;
use std::num;
use std::path::Path;
use std::str;
use std::fmt;
use std::error;

/// A path and its attendant 32-bit checksum.
#[derive(Debug)]
pub struct SfvRecord {
    path: String,
    checksum: u32,
}

impl SfvRecord {
    /// Creates a new `SfvRecord` based on a path.
    ///
    /// Bear in mind that creating an `SfvRecord` will, in fact, hash the file 
    /// in question, which can wreck your disk io, destroy your processor, and 
    /// (obviously) eat your laundry. However, there is no chance of nasal 
    /// demons with this function, because in the event that anything goes wrong,
    /// it will simply return an `io::Error`. 
    pub fn from_path<T: AsRef<Path> + Into<String>>(path: T) -> Result<SfvRecord, io::Error> {
        use std::fs::File;

        let mut file = File::open(path.as_ref())?;
        let mut digest = Crc32Digest::new();

        io::copy(&mut file, &mut digest)?;

        Ok(SfvRecord {
            path: path.into(),
            checksum: digest.hash(),
        })
    }

    /// Prints the `SfvRecord` to the provided writer.
    ///
    /// This is used by the `SfvCreator` to produce the actual sfv file.
    pub fn write<T: io::Write>(&self, writer: &mut T) -> Result<(), io::Error> {
        write!(writer, "{} {}\n", self.path, self.checksum)
    }

    /// Validates an `SfvRecord` against the file referenced by its path.
    ///
    /// If the file is not found or cannot be read, it will not validate. This 
    /// is used primarily by `Validator`.
    pub fn validate(&self) -> bool {
        use std::fs::File;

        let mut file = match File::open(&self.path) {
            Err(_) => { return false; },
            Ok(file) => file,
        };

        let mut digest = Crc32Digest::new();
        match io::copy(&mut file, &mut digest) {
            Err(_) => false,
            Ok(_) => digest == self.checksum
        }
    }
}

/// An error in parsing an `SfvRecord`.
#[derive(Debug)]
pub enum ParseSfvRecordError {
    MissingFilePath, 
    MissingChecksum,
    InvalidChecksum(num::ParseIntError),
    TooLong,
}

impl From<num::ParseIntError> for ParseSfvRecordError {
    fn from(error: num::ParseIntError) -> Self {
        ParseSfvRecordError::InvalidChecksum(error)
    }
}

impl fmt::Display for ParseSfvRecordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseSfvRecordError::MissingFilePath => write!(f, "Record has no file path"),
            ParseSfvRecordError::MissingChecksum => write!(f, "Record has no checksum"),
            ParseSfvRecordError::InvalidChecksum(ref e) => write!(f, "Invalid checksum: {}", e),
            ParseSfvRecordError::TooLong => write!(f, "Record has too many parts"),
        }
    }
}

impl error::Error for ParseSfvRecordError {
    fn description(&self) -> &str {
        match *self {
            ParseSfvRecordError::MissingFilePath => "Record has no file path",
            ParseSfvRecordError::MissingChecksum => "Record has no checksum",
            ParseSfvRecordError::InvalidChecksum(_) => "Invalid checksum",
            ParseSfvRecordError::TooLong => "Record has too many parts",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ParseSfvRecordError::MissingFilePath => None,
            ParseSfvRecordError::MissingChecksum => None,
            ParseSfvRecordError::InvalidChecksum(ref e) => Some(e),
            ParseSfvRecordError::TooLong => None,
        }
    }
}

impl str::FromStr for SfvRecord {
    type Err = ParseSfvRecordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let left = s.next().ok_or(ParseSfvRecordError::MissingFilePath)?.into();
        let right = s.next().ok_or(ParseSfvRecordError::MissingChecksum)?.to_lowercase();

        match s.next() {
            None => Ok(SfvRecord {
                path: left,
                checksum: u32::from_str_radix(&right, 16)?,
            }),

            _ => Err(ParseSfvRecordError::TooLong)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn well_formed_records_are_parsed() {
        let test_records = &[
            "file_one.zip   c45ad668",
            "file_two.zip   7903b8e6   ",
            "file_three.zip e99a65fb",
        ];

        assert!(test_records.iter().all(|record| record.parse::<SfvRecord>().is_ok()));
    }
}

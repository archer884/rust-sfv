use crc32::Crc32Digest;
use std::io;
use std::num;
use std::path::Path;
use std::str;

#[derive(Debug)]
pub struct SfvRecord {
    path: String,
    checksum: u32,
}

impl SfvRecord {
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

    pub fn write<T: io::Write>(&self, writer: &mut T) -> Result<(), io::Error> {
        write!(writer, "{} {}\n", self.path, self.checksum)
    }

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

        test_records.iter().all(|record| record.parse::<SfvRecord>().is_ok());
    }
}

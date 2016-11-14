use crc32::Crc32Digest;
use std::io;
use std::path::Path;
use std::str;

#[derive(Debug)]
pub struct SfvRecord {
    path: String,
    checksum: String,
}

impl SfvRecord {
    pub fn from_path<T: AsRef<Path> + Into<String>>(path: T) -> Result<SfvRecord, io::Error> {
        use std::fs::File;

        let mut file = File::open(path.as_ref())?;
        let mut digest = Crc32Digest::new();

        digest.update(&mut file);

        Ok(SfvRecord {
            path: path.into(),
            checksum: digest.to_string(),
        })
    }

    pub fn write<T: io::Write>(&self, writer: &mut T) -> Result<(), io::Error> {
        write!(writer, "{} {}\n", self.path, self.checksum)
    }

    pub fn validate(&self) -> bool {
        use std::fs::File;

        if let Ok(mut file) = File::open(&self.path) {
            let mut digest = Crc32Digest::new();
            digest.update(&mut file);
            return self.checksum.to_lowercase() == format!("{:x}", digest.value())
        }

        false
    }
}

pub enum ParseSfvRecordError {
    FilePath,
    Checksum,
    TooLong,
}

impl str::FromStr for SfvRecord {
    type Err = ParseSfvRecordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let left = s.next().ok_or(ParseSfvRecordError::FilePath)?.into();
        let right = s.next().ok_or(ParseSfvRecordError::Checksum)?.into();

        match s.next() {
            None => Ok(SfvRecord {
                path: left,
                checksum: right,
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

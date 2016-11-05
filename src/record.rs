use crc32::Crc32Digest;
use std::str;

pub struct FsvRecord {
    path: String,
    checksum: String,
}

impl FsvRecord {
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

pub enum ParseFsvRecordError {
    FilePath,
    Checksum,
    TooLong,
}

impl str::FromStr for FsvRecord {
    type Err = ParseFsvRecordError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();

        let left = match s.next() {
            None => return Err(ParseFsvRecordError::FilePath),
            Some(path) => path.into(),
        };

        let right = match s.next() {
            None => return Err(ParseFsvRecordError::Checksum),
            Some(checksum) => checksum.into(),
        };

        match s.next() {
            None => Ok(FsvRecord {
                path: left,
                checksum: right,
            }),

            _ => Err(ParseFsvRecordError::TooLong) 
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn well_formed_records_are_parsed() {
        let test_records = &[
            "file_one.zip   c45ad668",
            "file_two.zip   7903b8e6   ",
            "file_three.zip e99a65fb",
        ];

        test_records.iter().all(|record| record.parse::<super::FsvRecord>().is_ok());
    }
}

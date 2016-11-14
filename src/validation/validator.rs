use record::SfvRecord;
use validation::Result;

pub struct Validator {
    records: Vec<SfvRecord>
}

impl Validator {
    pub fn from_path(path: &str) -> Result {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(path).map(BufReader::new)?;
        let mut records = Vec::new();

        for line in file.lines() {
            let line = line?;

            if line.starts_with(';') {
                continue;
            }

            records.push(line.parse()?);
        }

        Ok(Validator { records: records })
    }

    pub fn validate(&self) -> bool {
        // TODO: ask what impact this is going to have
        #[inline(always)]
        fn validate(record: &SfvRecord) -> bool {
            record.validate()
        }

        self.records.iter().all(validate)
    }
}

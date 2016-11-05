extern crate crc;

mod crc32;
mod record;

pub fn validate(path: &str) -> bool {
    use record::FsvRecord;
    use std::io::{BufRead, BufReader};
    use std::fs::File;

    let input = match File::open(path) {
        Err(_) => return false,
        Ok(file) => BufReader::new(file),
    };

    let mut records: Vec<FsvRecord> = Vec::new();

    for line in input.lines() {
        match line {
            Err(_) => { return false; }
            Ok(line) => {
                if line.starts_with(';') {
                    continue;
                }

                match line.parse() {
                    Err(_) => { return false; },
                    Ok(record) => records.push(record),
                }
            }
        }
    }

    records.iter().all(|record| record.validate())
}

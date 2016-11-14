use record::SfvRecord;
use std::io;
use std::path::Path;

#[derive(Debug, Default)]
pub struct SfvCreator {
    records: Vec<SfvRecord>
}

impl SfvCreator {
    pub fn new() -> SfvCreator {
        SfvCreator::default()
    }

    pub fn add_path<T: AsRef<Path> + Into<String>>(&mut self, path: T) -> Result<(), io::Error> {
        Ok(self.records.push(SfvRecord::from_path(path)?))
    }

    pub fn write<T: io::Write>(&self, writer: &mut T) -> Result<(), io::Error> {
        write!(writer, ";created using rust-sfv\n")?;

        for record in &self.records {
            record.write(writer)?
        }

        Ok(())
    }
}

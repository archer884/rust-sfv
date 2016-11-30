use record::SfvRecord;
use std::io;
use std::path::Path;

/// Stores file paths and checksums for use in sfv creation.
#[derive(Debug, Default)]
pub struct SfvCreator {
    records: Vec<SfvRecord>
}

impl SfvCreator {
    /// Creates a new blank `SfvCreator`.
    pub fn new() -> SfvCreator {
        SfvCreator::default()
    }

    /// Adds a path to the sfv being created.
    pub fn add_path<T: AsRef<Path> + Into<String>>(&mut self, path: T) -> Result<(), io::Error> {
        Ok(self.records.push(SfvRecord::from_path(path)?))
    }

    /// Prints the formated sfv to the provided writer.
    pub fn write<T: io::Write>(&self, writer: &mut T) -> Result<(), io::Error> {
        write!(writer, ";created using rust-sfv\n")?;

        for record in &self.records {
            record.write(writer)?
        }

        Ok(())
    }
}

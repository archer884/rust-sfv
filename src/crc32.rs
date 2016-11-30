use std::fmt;
use std::io;

/// A 32-bit CRC digest.
#[derive(Copy, Clone, Default)]
pub struct Crc32Digest(u32);

impl Crc32Digest {
    /// Creates a new digest.
    pub fn new() -> Crc32Digest {
        Crc32Digest::default()
    }

    /// Returns the state of the digest as a `u32` value.
    pub fn hash(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for Crc32Digest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl fmt::Debug for Crc32Digest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl PartialEq<u32> for Crc32Digest {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl io::Write for Crc32Digest {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        use crc::crc32;
        
        self.0 = crc32::update(self.0, &crc32::IEEE_TABLE, buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
} 

#[cfg(test)]
mod tests {
    use std::io;
    use crc32::Crc32Digest;
    use crc::crc32::checksum_ieee;

    #[test]
    fn enhanced_digest_works() {
        let content = b"123456789";
        let mut cursor = io::Cursor::new(content);
        let mut digest = Crc32Digest::new();

        let _ = io::copy(&mut cursor, &mut digest);

        assert_eq!(checksum_ieee(content), digest.hash());
    }
}

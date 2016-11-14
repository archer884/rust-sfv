use std::fmt;
use std::io;

#[derive(Copy, Clone, Default)]
pub struct Crc32Digest(u32);

impl Crc32Digest {
    pub fn new() -> Crc32Digest {
        Crc32Digest::default()
    }

    pub fn update<T: io::Read>(&mut self, bytes: &mut T) {
        use crc::crc32;

        // TODO: Ask why this works?!
        let buf = &mut [0u8; 8192];
        loop {
            match bytes.read(buf) {
                Ok(0) => break,
                Ok(bytes_read) => self.0 = crc32::update(self.0, &crc32::IEEE_TABLE, &buf[0..bytes_read]),

                // I'd rather not panic, but there is really no valid response to this
                _ => break,
            }
        }
    }

    pub fn value(&self) -> u32 {
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

        digest.update(&mut cursor);

        assert_eq!(checksum_ieee(content), digest.value());
    }
}

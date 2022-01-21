use std::fmt;

pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
}

impl fmt::Display for ClassFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\tmagic: {:x}\n\tminor version: {}\n\tmajor version: {}\n\tconstant_pool_count: {}",
            self.magic,
            self.minor_version,
            self.major_version,
            self.constant_pool_count
        )
    }
}

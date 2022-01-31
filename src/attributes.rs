use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub enum Attributes {
    ConstantValue { const_idx: usize },
}

impl Attributes {
    pub fn parse_attribute(&mut self, byte_rdr: &mut Cursor<Vec<u8>>) {
        use Attributes::*;

        match self {
            ConstantValue { const_idx: idx } => {
                let curr = byte_rdr.position();
                byte_rdr.set_position(curr + 6);
                *idx = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;
            },
        }
    }
}

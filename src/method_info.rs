use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use crate::attributes::Attributes;

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_idx: usize,
    pub descriptor_idx: usize,
    pub attributes_count: usize,
    pub attributes: Vec<Attributes>,
}

impl MethodInfo {
    pub fn new() -> MethodInfo {
        MethodInfo {
            access_flags: 0,
            name_idx: 0,
            descriptor_idx: 0,
            attributes_count: 0,
            attributes: vec!(),
        }
    }

    pub fn parse_field(&mut self, byte_rdr: &mut Cursor<Vec<u8>>) {
        self.access_flags = byte_rdr.read_u16::<BigEndian>().unwrap();
        self.name_idx = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;
        self.descriptor_idx = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;
        self.attributes_count = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;
        self.attributes.reserve_exact(self.attributes_count);

        // for _ in 0..self.attributes_count {
        //     let mut attr = Attributes::ConstantValue { const_idx: 0 };
        //     attr.parse_attribute(byte_rdr);
        //     self.attributes.push(attr);
        // }
    }
}

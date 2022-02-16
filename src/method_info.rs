use core::panic;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use crate::{attributes::Attributes, constant_pool::ConstantPool};

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_idx: usize,
    pub descriptor_idx: usize,
    pub attributes_count: usize,
    pub attributes: Vec<Attributes>,
}

impl MethodInfo {
    pub fn new(byte_rdr: &mut Cursor<Vec<u8>>, cp: &ConstantPool) -> MethodInfo {
        let mut attr_count: usize = 0;
        MethodInfo {
            access_flags: byte_rdr.read_u16::<BigEndian>().unwrap(),
            name_idx: byte_rdr.read_u16::<BigEndian>().unwrap() as usize,
            descriptor_idx: byte_rdr.read_u16::<BigEndian>().unwrap() as usize,
            attributes_count: {
                attr_count = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;
                // According to The JVM spec 1.0.2 method_info should have no more than
                // two attributes: Code and Exceptions
                if attr_count < 1 && attr_count > 2 {
                    panic!("method_info has {} attributes instead of at most two: Code and Exceptions", attr_count);
                }
                attr_count
            },
            attributes: {
                let mut attr: Vec<Attributes> = Vec::new();
                attr.reserve_exact(attr_count);
                println!("Number of method_info attributes is {}", attr_count);
                for _ in 0..attr_count {
                    attr.push(Attributes::parse_attribute(byte_rdr, cp));
                }
                attr
            },
        }
    }
}

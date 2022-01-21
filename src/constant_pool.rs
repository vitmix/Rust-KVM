use std::{io::Cursor};
use byteorder::{BigEndian, ReadBytesExt};
use std::str;
use std::fmt;

pub enum ConstantPoolEntry {
    Utf8 (String),
    Integer (i32),
    Float (f32),
    Long (i64),
    Double (f64),
    Class { name_idx: u16 },
    String { string_idx: u16 },
    FieldRef { class_idx: u16, name_and_type_idx: u16 },
    MethodRef { class_idx: u16, name_and_type_idx: u16 },
    InterfaceMethodRef { class_idx: u16, name_and_type_idx: u16 },
    NameAndType { name_idx: u16, descriptor_idx: u16 },
}

impl From<u8> for ConstantPoolEntry {
    fn from(cp_tag: u8) -> Self {
        use ConstantPoolEntry::*;

        match cp_tag {
            7 => Class { name_idx: 0 },
            9 => FieldRef { class_idx: 0, name_and_type_idx: 0 },
            10 => MethodRef { class_idx: 0, name_and_type_idx: 0 },
            11 => InterfaceMethodRef { class_idx: 0, name_and_type_idx: 0 },
            8 => String { string_idx: 0 },
            3 => Integer (0),
            4 => Float (0.),
            5 => Long (0),
            6 => Double (0.),
            12 => NameAndType { name_idx: 0, descriptor_idx: 0 },
            1 => Utf8 (std::string::String::new()),
            _ => unreachable!("Unknown ConstantPool tag")
        }
    }
}

impl fmt::Display for ConstantPoolEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ConstantPoolEntry::*;

        match self {
            Class { name_idx: idx } => write!(f, "Class\t#{}", idx),
            FieldRef { class_idx: idx0, name_and_type_idx: idx1 } => write!(f, "Fieldref\t#{}.#{}", idx0, idx1),
            MethodRef { class_idx: idx0, name_and_type_idx: idx1 } => write!(f, "Methodref\t#{}.#{}", idx0, idx1),
            InterfaceMethodRef { class_idx: idx0, name_and_type_idx: idx1 } => write!(f, "InterfaceMethodref\t#{}.#{}", idx0, idx1),
            String { string_idx: idx } => write!(f, "String\t#{}", idx),
            Integer (i) => write!(f, "Integer\t{}", i),
            Float (fl) => write!(f, "Float\t{}", fl),
            Long (l) => write!(f, "Long\t{}", l),
            Double (d) => write!(f, "Double\t{}", d),
            NameAndType { name_idx: idx0, descriptor_idx: idx1 } => write!(f, "NameAndType\t#{}.#{}", idx0, idx1),
            Utf8 (s) => write!(f, "Utf8\t{}", s),
        }
    }
}

impl ConstantPoolEntry {
    pub fn parse_entry(&mut self, bytes: &[u8]) -> usize {
        use ConstantPoolEntry::*;

        let mut bytes_rdr = Cursor::new(bytes);
        match self {
            Class { name_idx: idx }
            | String { string_idx: idx } => {
                assert_eq!(bytes.len() >= 2, true);
                *idx = bytes_rdr.read_u16::<BigEndian>().unwrap();
                2
            },
            FieldRef { class_idx: idx0, name_and_type_idx: idx1 }
            | MethodRef { class_idx: idx0, name_and_type_idx: idx1 }
            | InterfaceMethodRef { class_idx: idx0, name_and_type_idx: idx1 }
            | NameAndType { name_idx: idx0, descriptor_idx: idx1 } => {
                assert_eq!(bytes.len() >= 4, true);
                *idx0 = bytes_rdr.read_u16::<BigEndian>().unwrap();
                *idx1 = bytes_rdr.read_u16::<BigEndian>().unwrap();
                4
            },
            Integer (value) => {
                assert_eq!(bytes.len() >= 4, true);
                *value = bytes_rdr.read_i32::<BigEndian>().unwrap();
                4
            },
            Float (value) => {
                assert_eq!(bytes.len() >= 4, true);
                *value = bytes_rdr.read_f32::<BigEndian>().unwrap();
                4
            },
            Long (value) => {
                assert_eq!(bytes.len() >= 8, true);
                *value = bytes_rdr.read_i64::<BigEndian>().unwrap();
                8
            },
            Double (value) => {
                assert_eq!(bytes.len() >= 8, true);
                *value = bytes_rdr.read_f64::<BigEndian>().unwrap();
                8
            },
            Utf8 (value) => {
                assert_eq!(bytes.len() >= 2, true);
                let length = bytes_rdr.read_u16::<BigEndian>().unwrap() as usize;
                *value = str::from_utf8(&bytes[2..length + 2]).unwrap().to_owned();
                2 + length
            },
            _ => unreachable!("Cannot parse entry of unknown constant pool tag")
        }
    }
}

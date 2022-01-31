use std::{io::Cursor};
use byteorder::{BigEndian, ReadBytesExt};
use std::str;
use std::fmt;

pub type ConstantPool = Vec<ConstantPoolEntry>;

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
            Utf8 (s) => write!(f, "{}", s),
        }
    }
}

impl ConstantPoolEntry {
    pub fn parse_entry(&mut self, byte_rdr: &mut Cursor<Vec<u8>>) {
        use ConstantPoolEntry::*;

        match self {
            Class { name_idx: idx }
            | String { string_idx: idx } => {
                *idx = byte_rdr.read_u16::<BigEndian>().unwrap();
            },
            FieldRef { class_idx: idx0, name_and_type_idx: idx1 }
            | MethodRef { class_idx: idx0, name_and_type_idx: idx1 }
            | InterfaceMethodRef { class_idx: idx0, name_and_type_idx: idx1 }
            | NameAndType { name_idx: idx0, descriptor_idx: idx1 } => {
                *idx0 = byte_rdr.read_u16::<BigEndian>().unwrap();
                *idx1 = byte_rdr.read_u16::<BigEndian>().unwrap();
            },
            Integer (value) => {
                *value = byte_rdr.read_i32::<BigEndian>().unwrap();
            },
            Float (value) => {
                *value = byte_rdr.read_f32::<BigEndian>().unwrap();
            },
            Long (value) => {
                *value = byte_rdr.read_i64::<BigEndian>().unwrap();
            },
            Double (value) => {
                *value = byte_rdr.read_f64::<BigEndian>().unwrap();
            },
            Utf8 (value) => {
                let length = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;
                let start = byte_rdr.position() as usize;
                let end = start + length;
                let refer = byte_rdr.get_ref();
                *value = str::from_utf8(&refer[start..end]).unwrap().to_owned();
                byte_rdr.set_position(end as u64);
            },
            _ => unreachable!("Cannot parse entry of unknown constant pool tag")
        }
    }
}

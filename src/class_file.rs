use std::fmt::{self};
use byteorder::{BigEndian, ReadBytesExt, ByteOrder};
use std::io::Cursor;

use crate::constant_pool::{ConstantPoolEntry, ConstantPool};
use crate::field_info::FieldInfo;
use crate::method_info::MethodInfo;

pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: usize,
    pub cp: ConstantPool,
    pub access_flags: u16,
    pub this_class: usize,
    pub super_class: usize,
    pub super_interfaces: Vec<usize>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
}

impl ClassFile {
    pub fn new() -> ClassFile {
        ClassFile {
            magic: 0,
            minor_version: 0,
            major_version: 0,
            constant_pool_count: 0,
            cp: vec!(),
            access_flags: 0,
            this_class: 0,
            super_class: 0,
            super_interfaces: vec!(),
            fields: vec!(),
            methods: vec!(),
        }
    }

    fn get_class_entry(&self, class_idx: usize) -> Result<&ConstantPoolEntry, &str> {
        match self.cp[class_idx] {
            ConstantPoolEntry::Class { name_idx: i } => Ok(&self.cp[i as usize]),
            _ => Err("Unknown constant pool entry"),
        }
    }

    pub fn parse(&mut self, byte_rdr: &mut Cursor<Vec<u8>>) {
        self.parse_class_header(byte_rdr);
        self.parse_constant_pool(byte_rdr);
        self.parse_fields(byte_rdr);
        self.parse_methods(byte_rdr);
    }

    fn parse_class_header(&mut self, byte_rdr: &mut Cursor<Vec<u8>>) {
        self.magic = byte_rdr.read_u32::<BigEndian>().unwrap();
        self.minor_version = byte_rdr.read_u16::<BigEndian>().unwrap();
        self.major_version = byte_rdr.read_u16::<BigEndian>().unwrap();
        self.constant_pool_count = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;
        assert!(self.constant_pool_count > 0);
    }

    fn parse_constant_pool(&mut self, byte_rdr: &mut Cursor<Vec<u8>>) {
        use ConstantPoolEntry::*;
        
        self.cp.reserve_exact(self.constant_pool_count);
        self.cp.push(Integer(0));

        for i in 1..(self.constant_pool_count) {
            let mut cp_entry = ConstantPoolEntry::from(byte_rdr.read_u8().unwrap());
            cp_entry.parse_entry(byte_rdr);
            self.cp.push(cp_entry);
            println!("\t#{} = {}", i, self.cp[i]);
        }

        assert_eq!(self.cp.len(), self.constant_pool_count);

        self.access_flags = byte_rdr.read_u16::<BigEndian>().unwrap();
        self.this_class = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;
        self.super_class = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;

        let class_name = self.get_class_entry(self.this_class).unwrap();
        println!("\tthis_class: #{} \t//{}", self.this_class, class_name);
        let super_name = self.get_class_entry(self.super_class).unwrap();
        println!("\tsuper_class: #{} \t//{}", self.super_class, super_name);

        let super_interfaces_count = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;
        self.super_interfaces.reserve_exact(super_interfaces_count);
        
        for i in 0..super_interfaces_count {
            self.super_interfaces.push(
                byte_rdr.read_u16::<BigEndian>().unwrap() as usize
            );
        }
        println!("Super interfaces: {:?}", self.super_interfaces);
    }

    fn parse_fields(&mut self, byte_rdr: &mut Cursor<Vec<u8>>) {
        let fields_count = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;
        self.fields.reserve_exact(fields_count);

        for _ in 0..fields_count {
            let mut field = FieldInfo::new();
            field.parse_field(byte_rdr, &self.cp);
            self.fields.push(field);
        }

        println!("Fields:\n\t{:?}", self.fields);
    }

    fn parse_methods(&mut self, byte_rdr: &mut Cursor<Vec<u8>>) {
        let methods_count = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;
        self.methods.reserve_exact(methods_count);
        println!("Methods count is {}", methods_count);
        for _ in 0..methods_count {
            self.methods.push(MethodInfo::new(byte_rdr, &self.cp));
        }
        println!("Methods:\n\t{:?}", self.methods);
    }
}

impl fmt::Display for ClassFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\tmagic: {:x}\n\tminor version: {}\n\tmajor version: {}\n\tconstant_pool_count: {}\n\taccess_flags: {}",
            self.magic,
            self.minor_version,
            self.major_version,
            self.constant_pool_count,
            self.access_flags
        )
    }
}

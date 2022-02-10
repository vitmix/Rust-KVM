use std::io::{Cursor, Read};
use byteorder::{BigEndian, ReadBytesExt};

use crate::constant_pool::*;

#[derive(Debug)]
pub enum Attributes {
    Undefined,
    ConstantValue { const_idx: usize },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<ExceptionInfo>,
    },
}

#[derive(Debug)]
pub struct ExceptionInfo {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl ExceptionInfo {
    pub fn new(byte_rdr: &mut Cursor<Vec<u8>>) -> ExceptionInfo {
        ExceptionInfo {
            start_pc: byte_rdr.read_u16::<BigEndian>().unwrap(),
            end_pc: byte_rdr.read_u16::<BigEndian>().unwrap(),
            handler_pc: byte_rdr.read_u16::<BigEndian>().unwrap(),
            catch_type: byte_rdr.read_u16::<BigEndian>().unwrap(),
        }
    }
}

impl Attributes {
    pub fn parse_attribute(byte_rdr: &mut Cursor<Vec<u8>>, cp: &ConstantPool) -> Attributes {
        use Attributes::*;

        let attr_name_idx = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;
        let attr_name = match &cp[attr_name_idx] {
            ConstantPoolEntry::Utf8 (name) => name,
            _ => panic!("There is no attribute name for index {}", attr_name_idx),
        };

        match attr_name.as_str() {
            "ConstantValue" => {
                let curr = byte_rdr.position();
                byte_rdr.set_position(curr + 4);
                ConstantValue { const_idx: byte_rdr.read_u16::<BigEndian>().unwrap() as usize }
            },
            "Code" => {
                let code_attr_len = byte_rdr.read_u32::<BigEndian>().unwrap() as u64;
                let code_attr = Code {
                    max_stack: byte_rdr.read_u16::<BigEndian>().unwrap(),
                    max_locals: byte_rdr.read_u16::<BigEndian>().unwrap(),
                    code: {
                        let code_length = byte_rdr.read_u32::<BigEndian>().unwrap() as usize;
                        let start = byte_rdr.position() as usize;
                        let end = start + code_length;
                        let code_slice = byte_rdr.get_ref();
                        let bytecode = code_slice[start..end].to_vec();
                        byte_rdr.set_position(end as u64);
                        bytecode
                    },
                    exception_table: {
                        let exc_table_len = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;
                        let mut excs: Vec<ExceptionInfo> = Vec::new();
                        excs.reserve_exact(exc_table_len);
                        
                        for _ in 0..exc_table_len {
                            excs.push(ExceptionInfo::new(byte_rdr));
                        }
                        excs
                    },
                };
                // skipping Code`s attributes (LineNumberTable, LocalVariableTable)
                byte_rdr.set_position(code_attr_len);
                code_attr
            },
            _ => Undefined
        }

        //cp[attr_name_idx]

        // match self.cp[class_idx] {
        //     ConstantPoolEntry::Class { name_idx: i } => Ok(&self.cp[i as usize]),
        //     _ => Err("Unknown constant pool entry"),
        // }

        // match self {
        //     ConstantValue { const_idx: idx } => {
        //         let curr = byte_rdr.position();
        //         byte_rdr.set_position(curr + 6);
        //         *idx = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;
        //     },
        //     Code 
        // }
    }
}

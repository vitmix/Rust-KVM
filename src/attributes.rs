use std::io::{Cursor, Read};
use byteorder::{BigEndian, ReadBytesExt};

use crate::constant_pool::*;

#[derive(Debug)]
pub enum Attributes {
    ConstantValue { const_idx: usize },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<ExceptionHandlingInfo>,
    },
    Exceptions {
        exception_ids: Vec<u16>,
    },
}

#[derive(Debug)]
pub struct ExceptionHandlingInfo {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl ExceptionHandlingInfo {
    pub fn new(byte_rdr: &mut Cursor<Vec<u8>>) -> ExceptionHandlingInfo {
        ExceptionHandlingInfo {
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
        println!("Attribute name index {}", attr_name_idx);
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
                let curr_rdr_pos = byte_rdr.position();

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
                        let mut excs: Vec<ExceptionHandlingInfo> = Vec::new();
                        excs.reserve_exact(exc_table_len);
                        
                        for _ in 0..exc_table_len {
                            excs.push(ExceptionHandlingInfo::new(byte_rdr));
                        }
                        excs
                    },
                };
                // skipping Code`s attributes (LineNumberTable, LocalVariableTable)
                byte_rdr.set_position(curr_rdr_pos + code_attr_len);
                code_attr
            },
            "Exceptions" => {
                let curr = byte_rdr.position();
                byte_rdr.set_position(curr + 4);
                let num_of_excs = byte_rdr.read_u16::<BigEndian>().unwrap() as usize;
                let mut exc_ids: Vec<u16> = Vec::new();
                exc_ids.reserve_exact(num_of_excs);

                for _ in 0..num_of_excs {
                    exc_ids.push(byte_rdr.read_u16::<BigEndian>().unwrap());
                }
                Exceptions { exception_ids: exc_ids }
            },
            _ => panic!("Unknown attribute name was provided!"),
        }
    }
}

use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

use crate::class_file::ClassFile as ClassFile;
use crate::constant_pool::ConstantPoolEntry;

pub fn read_as_bytes(file_path: &Path) -> Result<Vec<u8>, std::io::Error> {
    File::open(file_path).and_then(|mut file| {
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    })
}

pub fn is_java_class_format(binary: &[u8]) -> bool {
    match binary {
        [0xCA, 0xFE, 0xBA, 0xBE, ..] => true,
        _ => false,
    }
}

pub fn parse_class_header(bytecode: &[u8]) -> Result<ClassFile, std::io::Error> {
    assert_eq!(bytecode.len(), 10);
    let mut byte_rdr = Cursor::new(bytecode);
    
    Ok(ClassFile {
        magic: byte_rdr.read_u32::<BigEndian>().unwrap(),
        minor_version: byte_rdr.read_u16::<BigEndian>().unwrap(),
        major_version: byte_rdr.read_u16::<BigEndian>().unwrap(),
        constant_pool_count: byte_rdr.read_u16::<BigEndian>().unwrap(),
    })
}

pub fn read_raw_java_class_dumb(class_path: &Path) {
    println!("Trying to read Java`s class in bytecode");
    let bytecode = match read_as_bytes(class_path) {
        Ok(bytes) => bytes,
        _ => panic!("Error reading provided Java`s class: {}", class_path.display()),
    };
    println!("Is provided binary is Java`s class file format ? - {}", is_java_class_format(&bytecode));
    
    let mut read_idx: usize = 10;
    let mut cp_read_counter: u16 = 0;

    let jclass = match parse_class_header(&bytecode[0..read_idx]) {
        Ok(class) => class,
        _ => panic!("Unable to parse Java`s class header !.."),
    };
    println!("Parsed:\n{}", jclass);

    while cp_read_counter < jclass.constant_pool_count - 1 {
        cp_read_counter += 1;
        let mut cp_entry = ConstantPoolEntry::from(bytecode[read_idx]);
        read_idx += 1;
        let was_read = cp_entry.parse_entry(&bytecode[read_idx..]);
        println!("\t#{} = {}", cp_read_counter, cp_entry.name_and_value());
        read_idx += was_read;
    }
}
 
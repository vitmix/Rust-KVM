use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt, ByteOrder};

use crate::class_file::ClassFile as ClassFile;
use crate::constant_pool::{ConstantPoolEntry, ConstantPool};
use crate::declarations::{ACC_PUBLIC, ACC_FINAL, ACC_SUPER, ACC_INTERFACE, ACC_ABSTRACT};

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

pub fn read_raw_java_class_dumb(class_path: &Path) {
    println!("Trying to read Java`s class in bytecode");
    let bytecode = match read_as_bytes(class_path) {
        Ok(bytes) => bytes,
        _ => panic!("Error reading provided Java`s class: {}", class_path.display()),
    };
    println!("Is provided binary is Java`s class file format ? - {}", is_java_class_format(&bytecode));

    let mut jclass = ClassFile::new();
    let mut byte_rdr = Cursor::new(bytecode);
    jclass.parse(&mut byte_rdr);
    println!("Parsed:\n{}", jclass);
    
    // for acc_flag in [ACC_PUBLIC, ACC_FINAL, ACC_SUPER, ACC_INTERFACE, ACC_ABSTRACT] {
    //     if jclass.access_flags & acc_flag == acc_flag {
    //         println!("{:x} flag is present in access_flags {:x}", acc_flag, jclass.access_flags);
    //     }
    // }
}

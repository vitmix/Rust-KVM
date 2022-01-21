use std::path::Path;

use constant_pool::ConstantPoolEntry;

mod raw_class_reader;
mod class_file;
mod constant_pool;

fn main() {
    raw_class_reader::read_raw_java_class_dumb(Path::new("D:/My Java Projects/HelloWorld.class"));

    //let bytes: Vec<u8> = vec![2, 5, 3, 4, 5];
    //let slice: &[u8; 2] = &bytes[0..2];
    //println!("Interpret as u16 : {}", slice.read_u16::<BigEndian>());
    //println!("Interpret as u16 : {}", ((slice[0] as u16) << 8) + slice[1] as u16);
}

#[test]
fn test_cp_class_entry_parsing() {
    let bytes: Vec<u8> = vec![7, 1, 2, 3, 4, 5];
    let mut class_entry = ConstantPoolEntry::from(bytes[0]);

    if let ConstantPoolEntry::Class { name_idx: value } = class_entry {
        assert_eq!(0, value);
        println!("1. Value of Class::name_index is {}", value);
    }

    class_entry.parse_entry(&bytes[1..=2]);

    if let ConstantPoolEntry::Class { name_idx: value } = class_entry {
        assert_eq!(258, value);
        println!("2. Value of Class::name_index is {}", value);
    }
}

#[test]
fn test_cp_fieldref_entry_parsing() {
    let bytes: Vec<u8> = vec![9, 2, 5, 3, 0];
    let mut fieldref_entry = ConstantPoolEntry::from(bytes[0]);

    if let ConstantPoolEntry::FieldRef { class_idx: cidx, name_and_type_idx: ntidx } = fieldref_entry {
        assert_eq!(0, cidx);
        assert_eq!(0, ntidx);
        println!("Fieldref {} {}", cidx, ntidx);
    }

    fieldref_entry.parse_entry(&bytes[1..]);

    if let ConstantPoolEntry::FieldRef { class_idx: cidx, name_and_type_idx: ntidx } = fieldref_entry {
        println!("Fieldref {} {}", cidx, ntidx);
    }
    println!("Bytes vector is {:?}", bytes);
}

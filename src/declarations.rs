macro_rules! define_access_flag {
    ($name:ident, $value:expr) => {
        pub const $name: u16 = $value;
    };
}

define_access_flag!(ACC_PUBLIC, 0x0001);
define_access_flag!(ACC_FINAL, 0x0010);
define_access_flag!(ACC_SUPER, 0x0020);
define_access_flag!(ACC_INTERFACE, 0x0200);
define_access_flag!(ACC_ABSTRACT, 0x0400);

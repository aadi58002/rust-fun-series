extern crate break_macro;

use break_macro::*;

#[allow(unused)]
#[derive(Clear)]
struct Str {
    field1: usize,
    field2: String,
}

#[allow(unused)]
#[derive(Clear)]
enum Enu {
    Value1,
    Value2(usize),
    Value3(String),
}

#[test]
fn testing_clear_proc_macro() {
    assert_ne!(std::mem::size_of::<Str>(),0);
    assert_eq!(std::mem::size_of::<StrClear>(),0);

    assert_ne!(std::mem::size_of::<Enu>(),0);
    assert_eq!(std::mem::size_of::<EnuClear>(),0);
}


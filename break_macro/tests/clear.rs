#![allow(unused)]

extern crate break_macro;

use break_macro::Clear;

#[derive(Clear)]
struct StrNamed {
    field1: usize,
    field2: String,
}

#[derive(Clear)]
struct StrUnnamed(String, String, usize);

#[derive(Clear)]
struct StrUnit;

#[derive(Clear)]
enum EnumNamed {
    Value1,
    Value2 { x: usize },
    Value3 { y: String },
}

#[derive(Clear)]
enum EnumUnnamed {
    Value1,
    Value2(usize),
    Value3(String),
}

#[test]
fn testing_clear_proc_macro() {
    let _ = StrNamed {
        field1: 0,
        field2: "".to_string(),
    };
    let _ = StrNamedClear;

    let _ = StrUnnamed("".to_string(), "".to_string(), 0);
    let _ = StrUnnamedClear;

    let _ = StrNamed {
        field1: 0,
        field2: "".to_string(),
    };
    let _ = StrNamedClear;

    let _ = StrUnit;
    let _ = StrUnitClear;
}

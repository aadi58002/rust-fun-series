#![allow(unused)]

extern crate break_macro;
use break_macro::ClearString;

#[derive(ClearString)]
struct StrNamed {
    field1: usize,
    field2: String,
}

#[derive(ClearString)]
struct StrUnnamed(String, String, usize);

#[derive(ClearString)]
struct StrUnit;

#[derive(ClearString)]
enum EnumNamed {
    Value1,
    Value2 { x: usize },
    Value3 { y: String },
}

#[derive(ClearString)]
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
    let _ = StrNamedClearString { field1: 0 };

    let _ = StrUnnamed("".to_string(), "".to_string(), 0);
    let _ = StrUnnamedClearString(0);

    let _ = StrNamed {
        field1: 0,
        field2: "".to_string(),
    };
    let _ = StrNamedClearString { field1: 0 };

    let _ = StrUnit;
    let _ = StrUnitClearString;

    let test_enum = EnumNamed::Value1;
    match test_enum {
        EnumNamed::Value1 => {}
        EnumNamed::Value2 { x: _ } => {}
        EnumNamed::Value3 { y: _ } => {}
    }

    let test_enum = EnumNamedClearString::Value1;
    match test_enum {
        EnumNamedClearString::Value1 => {}
        EnumNamedClearString::Value2 { x: _ } => {}
        EnumNamedClearString::Value3 { y: _ } => {}
    }

    let test_enum = EnumUnnamed::Value1;
    match test_enum {
        EnumUnnamed::Value1 => {}
        EnumUnnamed::Value2(_) => {}
        EnumUnnamed::Value3(_) => {}
    }

    let test_enum = EnumUnnamedClearString::Value1;
    match test_enum {
        EnumUnnamedClearString::Value1 => {}
        EnumUnnamedClearString::Value2(_) => {}
    }
}

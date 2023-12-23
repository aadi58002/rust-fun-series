extern crate break_macro;

use break_macro::*;

#[allow(unused)]
#[derive(ClearString)]
struct StrNamed {
    field1: usize,
    field2: String,
}

#[allow(unused)]
#[derive(ClearString)]
struct StrUnnamed(String, String, usize);

#[allow(unused)]
#[derive(ClearString)]
struct StrUnit;

#[allow(unused)]
#[derive(ClearString)]
enum EnuNamed {
    Value2 { x: usize },
    Value3 { y: String },
}

#[allow(unused)]
#[derive(ClearString)]
enum EnuUnnamed {
    Value1,
    Value2(usize),
    Value3(String),
}

#[allow(unused)]
#[derive(ClearString)]
enum EnuUnit {}

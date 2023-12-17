extern crate break_macro;

use break_macro::*;

#[allow(unused)]
#[derive(Clear)]
struct Str {
    field1: usize,
    field2: String,
    field3: u64,
    field4: u32,
    field5: isize,
    field6: f64,
    field7: Option<usize>,
    field8: std::rc::Rc<usize>,
}

#[allow(unused)]
#[derive(Clear)]
enum Enu {
    Value1,
    Value2(usize),
    Value3(String),
    Value4((usize, usize)),
    Value5(Str),
    Value6(Option<usize>),
}

#[test]
fn testing_basic_proc_macro() {
    assert_ne!(std::mem::size_of::<Str>(),0);
    assert_eq!(std::mem::size_of::<StrEmpty>(),0);

    assert_ne!(std::mem::size_of::<Enu>(),0);
    assert_eq!(std::mem::size_of::<EnuEmpty>(),0);
}

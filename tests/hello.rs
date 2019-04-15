extern crate pm;
use pm::*;

#[derive(Debug, HelloMacro)]
struct Test {}

#[derive(Debug, HelloMacro, HelloFields)]
struct TestTwo {
    pub id: u8,
    pub name: String,
}

impl TestTwo {
    pub fn new(i: u8, n: String) -> TestTwo {
        TestTwo { id: i, name: n }
    }
}

#[test]
fn an_arbitrary_id_is_returned() {
    let id = Test::get_id();
    assert!(id == 9);
}

#[test]
fn two_structs_both_display_behavior() {
    assert!(Test::get_id() == TestTwo::get_id());
}

#[test]
fn a_struct_can_still_be_implemented_in_main() {
    let id = 10;
    let name = "foo".to_string();

    let tt = TestTwo::new(id, name);

    assert!(TestTwo::get_id() != tt.id);
}

#[test]
fn a_struct_print_the_field_name() {
    let id = 10;
    let name = "foo".to_string();

    let tt = TestTwo::new(id, name.clone());

    assert!(tt.print_name() == name);
}

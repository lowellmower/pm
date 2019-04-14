extern crate pm;
use pm::HelloMacro;

#[derive(Debug, HelloMacro)]
struct Test {}

#[derive(Debug, HelloMacro)]
struct TestTwo {}

#[test]
fn an_arbitrary_id_is_returned() {
    let id = Test::get_id();
    assert!(id == 9);
}

#[test]
fn two_structs_both_display_behavior() {
    assert!(Test::get_id() == TestTwo::get_id());
}

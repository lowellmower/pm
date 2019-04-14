extern crate pm;
use pm::HelloMacro;

#[derive(Debug, HelloMacro)]
struct Test {}

#[test]
fn works() {
    let id = Test::get_id();
    assert!(id == 9);
}

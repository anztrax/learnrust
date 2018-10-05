extern crate learnrust;

mod common;

#[test]
fn integration_it_adds_two() {
  common::setup();
  assert_eq!(4, learnrust::adder::add_two(2));
}

use ::lessons::Rectangle::Rectangle;
use ::lessons::Guess::Guess;

pub fn add_two(num: i32) -> i32{
  num + 2
}

pub fn greeting(name: &str) -> String{
  format!("hello, {}", name)
}

#[test]
fn exploration(){
  assert_eq!(2 + 2, 4);
}

#[test]
fn larger_can_hold_smaller(){
  let larger = Rectangle::new(8, 7);
  let smaller = Rectangle::new(5, 1);
  assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_can_not_hold_larger(){
  let larger = Rectangle::new(8, 7);
  let smaller = Rectangle::new(5, 1);
  assert!(!smaller.can_hold(&larger));
}

#[test]
#[ignore]
fn test_greeting_name(){
  let result = greeting("carol");
  assert!(
    result.contains("carol1"),
    "Greeting did not contain name, value was `{}`", result
  );
}

#[test]
#[should_panic(expected = "assertion failed")]
#[ignore]
fn test_guess_game(){
  let guess = Guess::new(0);

}

#[test]
fn add_two_test(){
  assert_eq!(4, add_two(2));
}

#[test]
#[ignore]
fn another(){
  panic!("Make this test fail !");
}

#[test]
#[ignore]
fn it_works_using_result() -> Result<(), String> {
  if 2 + 2 == 5 {
    Ok(())
  } else {
    Err(String::from("two plus two does not equal four"))
  }
}

#[test]
fn test_closure(){
  let x = 5;
  let same_as_y = |y|{ y == x };
  let y = 10;
  same_as_y(10);
  assert_eq!(same_as_y(10), false);

}


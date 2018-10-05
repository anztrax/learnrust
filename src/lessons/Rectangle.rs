#[derive(PartialEq, Debug)]
pub struct Rectangle {
  length: u32,
  width: u32,
}

impl Rectangle {
  pub fn new(length: u32, width: u32) -> Rectangle{
    Rectangle{
      length,
      width
    }
  }

  pub fn can_hold(&self, other: &Rectangle) -> bool {
    self.length > other.length && self.width > other.width
  }
}
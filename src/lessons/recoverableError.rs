use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;
use std::io;


struct Guess{
  value1: u32,
}

impl Guess{
  pub fn new(value: u32) -> Guess{
    if value < 1 || value > 100{
      panic!("Guess value must be between 1 and 100, got {}.", value);
    }
    Guess{
      value1 : value
    }
  }

  pub fn value(&self) -> u32{
    self.value1
  }
}

pub fn recoverableError(){
  let f = File::open("hello.txt");
  let t = match f{
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt"){
        Ok(fc) => fc,
        Err(e) => panic!("Tried to create file but there was a problem: {:?}", e)
      },
      _ => panic!("There was a problem opening the file"),
    }
  };

  use_closure();
  use_unwrap_or_expect();
  let result = read_username_from_file();
  println!("result read username from file :{:?}", result);
  let result = read_username_from_file2();
  println!("result read username from file :{:?}", result);
  let result = read_username_from_file_use_chaining_method();
  println!("result read username from file :{:?}", result);
  let result = fs::read_to_string("hello.txt");
  println!("result read username from file :{:?}", result);

  let guess1 = Guess::new(10);
  println!("guess1 value: {}", guess1.value());

  println!("file value : {:?}", t);
}

fn use_closure(){
  //using closure
  let f = File::open("hello.txt").map_err(|error| {
    if error.kind() == ErrorKind::NotFound{
      File::create("hello.txt").unwrap_or_else(|error|{
        panic!("Tried to create file but there was a problem: {:?}", error);
      });
    } else {
      panic!("There was a problem opening the file: {:?}", error);
    }
  });
  println!("file value : {:?}", f);
}

fn use_unwrap_or_expect(){
  let f = File::open("hello.txt").unwrap();
  println!("f value : {:?}", f);

  let f = File::open("hello.txt").expect("Failed to open hello.txt");
  println!("f value : {:?}", f);
}


fn read_username_from_file() -> Result<String, io::Error>{
  let f = File::open("hello.txt");

  let mut f = match f{
    Ok(file) => file,
    Err(e) => return Err(e)
  };

  let mut s = String::new();
  match f.read_to_string(&mut s){
    Ok(_) => Ok(s),
    Err(e) => Err(e)
  }
}

fn read_username_from_file2() -> Result<String, io::Error>{
  let mut f = File::open("hello.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}

fn read_username_from_file_use_chaining_method() -> Result<String, io::Error>{
  let mut s = String::new();
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}
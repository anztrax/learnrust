use std::thread;
use std::time::Duration;


pub struct Cacher<T> where T: Fn(u32) -> u32{
  calculation:T,
  value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32{
  fn new(calculation: T) -> Cacher<T>{
    Cacher{
      calculation,
      value: None
    }
  }

  fn value(&mut self, arg: u32) -> u32{
    match self.value{
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

///
/// try using documentation comment,
/// this is handy bro
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn simulated_expensive_calculation(intensity: u32){
  println!("calculating slowly");
  expensive_calculation(100);

  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(
    simulated_user_specified_value,
    simulated_random_number
  );
}

pub fn expensive_calculation(intensity: u32) -> u32{
  thread::sleep(Duration::from_secs(2));
  intensity
}



pub fn generate_workout(intensity: u32, random_number: u32){
  let expensive_result2 = expensive_calculation(intensity);
  let expensive_result3 = |num: u32| -> u32{
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  };

  let mut expensive_result = Cacher::new(|num| {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  //you can skip add type to closure
  let example_closure = |x| x;
  let x1 = example_closure(String::from("testing gan"));
  let x2 = example_closure(String::from("testing gan1 123"));
  println!("x1 : {}, x2: {}", x1, x2);


  if intensity < 25 {
    println!(
      "Today, do {} pushups!",
      expensive_result.value(intensity)
    );
    println!(
      "Next, do {} situps!",
      expensive_result.value(intensity)
    );
  } else{
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!(
        "Today, run for {} minutes!",
        expensive_result.value(intensity)
      );
    }
  }
}
#[derive(Debug)]
struct Shoe{
  size : u32,
  style: String
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>{
  shoes.into_iter()
    .filter(|s| s.size == shoe_size)
    .collect()
}

//-------------- create custom iterator things
struct Counter{
  count: u32
}

impl Counter{
  fn new() -> Counter{
    Counter{ count: 0 }
  }
}

impl Iterator for Counter{
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item>{
    self.count += 1;

    if(self.count < 6){
      Some(self.count)
    }else{
      None
    }
  }
}



//------------- main function
pub fn learnIterator(){
  let v1 = vec![1,2,3,4];
  let v1_iter = v1.iter();
  for item in v1_iter{
    println!("v1 iter : {}", item);
  }

  let v2 = vec![1,2,3,4,5];
  let mut v2_iter = v2.iter();
  match v2_iter.next(){
    Some(value) => println!("v2 iter 0 value : {}", value),
    None => println!("none !")
  }

  //iterator adaptors
  let v1: Vec<i32> = vec![1,2,3,4];
  let v1_result: Vec<i32> = v1.iter()
    .map(|x| x + 1) //this cause warning because didn't consume well
    .collect();

  println!("v1_result {:?}", v1_result);
  find_suitable_shoes();
  implement_iterator();
  println!("============================");
}

//shoe things
fn find_suitable_shoes(){
  let shoes = vec![
    Shoe{ size: 10, style: String::from("sneaker")},
    Shoe{ size: 20, style: String::from("sneaker")},
    Shoe{ size: 22, style: String::from("sneaker")},
  ];

  let in_my_size = shoes_in_my_size(shoes, 20);
  println!("in my size {:?}",in_my_size);
}

fn implement_iterator(){
  let mut counter = Counter::new();
  println!("value : {}", counter.next().unwrap());
  let sum: u32 = Counter::new().zip(Counter::new().skip(1))
    .map(|(a, b)| a * b )
    .filter(|x| x % 3 == 0)
    .sum();
  println!("value of sum is {}", sum);

}
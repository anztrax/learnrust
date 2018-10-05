#[derive(Debug)]
struct Point<T>{
  x: T,
  y: T
}

//implement generic type method
impl<T> Point<T>{
  fn x1(&self) -> &T{
    &self.x
  }
}

impl Point<u32>{
  fn distance_from_origin(&self) -> u32{
    (self.x.pow(2) +  self.y.pow(2))
  }
}

#[derive(Debug)]
struct Point2<X,Y> {
  x: X,
  y: Y
}

//implement when want to mixed up with different type of generic types
impl<X, Y> Point2<X, Y>{
  fn mixup<T,U>(self, other: Point2<T,U>) -> Point2<X, U>{
    Point2{
      x: self.x,
      y: other.y
    }
  }
}


pub fn try_generic_types(){
  println!("=============================");
  println!("try generic types !");

  let largest_number = largest(&(vec![10, 20, 50, 60, 100]));
  println!("largest number : {}",largest_number);

  let largest_number = largest(&(vec![10, 20, 50, 60, 100, 1000]));
  println!("largest number : {}",largest_number);

  let point1 = Point{x : 10, y : 20};
  let point2 = Point{x : 10.1, y: 20.2};
  point1.x1();
  println!("pow 2 of point1 is : {}", point1.distance_from_origin());

  let point2_1 = Point2{x : 10, y : 10.2};
  let point2_2 = Point2{x : 20, y : 20.1};
  let point2_3 = Point2{x : "hello", y: "there"};
  println!("point2 value is : {:?}", point2_3.mixup(point2_1));


  try_trait_things();

  let largest1 = largest_generic(&(vec!['a', 'b', 'z']));
  let largest2 = largest_generic(&(vec![10, 40, 60, 100, 1000]));
  println!("largest value 1 = {} ,  2 ={}", largest1, largest2);


  println!("=============================");
}

fn largest(list : &[u32]) -> u32{
  let mut largest = list[0];

  for &item in list.iter(){
    if item > largest{
      largest = item;
    }
  }
  largest
}


//trait
pub trait Summary{
  fn summarize_author(&self) -> String;
  fn summarize(&self) -> String;
  fn summarize_2(&self) -> String{
    format!("(Read More from {} ...)", self.summarize_author())
  }
}

pub struct NewsArticle{
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle{
  fn summarize_author(&self)-> String{
    format!("@{}", self.author)
  }

  fn summarize(&self) -> String{
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet{
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool
}

impl Summary for Tweet{
  fn summarize_author(&self)-> String{
    format!("@{}", self.username)
  }

  fn summarize(&self) -> String{
    format!("{}: {}", self.username, self.content)
  }
}

//notify user
pub fn notify(item: impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

pub fn notify_2<T: Summary>(item: T){
  println!("Breaking news! {}", item.summarize_2());
}

fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  }
}

//try traits
fn try_trait_things(){
  let tweet = Tweet{
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };

  let news_letter = NewsArticle{
    headline: String::from("news from headline"),
    location: String::from("New York"),
    author: String::from("andrew "),
    content: String::from("of course, as you probably already know, people"),
  };

  println!("1 new tweet {}", tweet.summarize());
  println!("1 new tweet {}", tweet.summarize_2());
  println!("1 new news article {}", news_letter.summarize_2());
  notify(news_letter);
  notify_2(tweet);
}

//largest function using generic
fn largest_generic<T:PartialOrd + Copy>(list: &[T]) -> T{
  let mut largest = list[0];

  for &item in list.iter(){
    if item > largest{
      largest = item;
    }
  }
  largest
}
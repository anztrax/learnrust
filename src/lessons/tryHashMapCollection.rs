use std::collections::HashMap;

pub fn tryHashMap(){
  println!("========================");

  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  println!("scores : {:?}", scores);

  //convert vector into hashmap
  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];

  let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
  println!("scores : {:?}", scores);

  let field_name = String::from("Favourite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);
  println!("map {:?}", map);


  //access value from scores
  let score = &scores.get(&String::from("Blue"));
  match score{
    Some(value) => println!("yeah there's a value {}", value),
    None => println!("is empty!")
  }

  //loop access value
  for (key , value) in &scores{
    println!("score {} : {}", key, value);
  }


  //update hashmap value
  let mut scores2 = HashMap::new();
  scores2.insert(String::from("Blue"), 10);
  scores2.insert(String::from("Blue"), 25);
  println!("scores2 : {:?}", scores2);

  //insert if key is not available
  scores2.entry(String::from("Yellow")).or_insert(50);
  scores2.entry(String::from("Blue")).or_insert(50);
  println!("scores2 : {:?}", scores2);

  split_text_in_map();
  println!("========================");
}



fn split_text_in_map(){
  let text = "hello world wonderful world";
  let mut map = HashMap::new();

  for word in text.split_whitespace(){
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);
}
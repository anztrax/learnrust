pub fn try_lifetime(){
  println!("=============================");
  println!("try lifetime");

  let x = 5;
  let r;
  r = &x;
  println!("r {}",r);

  let string1 = String::from("abcd");
  let string2 = "xyz";
  let result = longest(string1.as_str(), string2);
  println!("longest string {}", result);

  let string1 = String::from("long string is long");
  let string2 = String::from("xyz");
  let result= longest(string1.as_str(), string2.as_str());
  println!("The longest string is {}", result);

  println!("=============================");
}


fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str{
  if str1.len() > str2.len(){
    str1
  }else{
    str2
  }
}

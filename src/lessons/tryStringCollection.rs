pub fn tryUpdateString(){
  println!("==============================");

  let mut s = String::from("foo");
  s.push_str(" bar");
  println!("s {}", s);

  let s2 = "bar";
  s.push_str(s2);

  println!("s2 : {}",s2);

  let mut s = String::from("lo");
  s.push('l');
  println!("s : {}",s);

  let string1 = String::from("Hello, ");
  let string2 = String::from(" World !");
  let string3 = string1 + &string2;
  println!("string3 : {}", string3);

  //you can print string
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  let s = format!("{}-{}-{}", s1, s3,s3);
  println!("s value {}", s);

  let s1 = "test-Здравствуйте";
  let s1 = &s1[0..=4];
  println!("s1 value : {}", s1);

  for c in "नमस्ते".chars(){
    println!("{}",c);
  }

  println!("==============================");
}
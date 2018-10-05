extern crate rand;
extern crate learnrust;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use learnrust::lessons::commonCollection;
use learnrust::lessons::tryStringCollection;
use learnrust::lessons::tryHashMapCollection;
use learnrust::lessons::recoverableError;
use learnrust::lessons::tryGenericTypes;
use learnrust::lessons::tryLifetime;

use learnrust::closures::learnClosure;
use learnrust::closures::learnIterator;

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    learnIterator::learnIterator();
    learnClosure::simulated_expensive_calculation(10);

    tryLifetime::try_lifetime();
    tryGenericTypes::try_generic_types();
    recoverableError::recoverableError();
    tryHashMapCollection::tryHashMap();
    tryStringCollection::tryUpdateString();
    commonCollection::tryCommonCollection();


    try_struct();
    reference_and_borrowing();
    ownership_in_rust();
    try_control_flow();
    another_func(100);
    learn_variable();
    learn_datatype();

    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line !");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too big !"),
            Ordering::Equal => {
                println!("You win !");
                break;
            }
        }
    }
}

fn try_struct(){
    println!("==============================================================================");

    let mut user1 = User{
        active: true,
        email : String::from("test@test.com"),
        sign_in_count: 10,
        username: String::from("test.test")
    };

    println!("user1 : {}", user1.username);
    user1.username = String::from("hello world");
    println!("user1 : {}", user1.username);

    let user2 = create_user(String::from("user2@user2.com"), String::from("user2"));
    println!("user2 : {}", user2.username);

    let user3 = User{
        ..user2
    };
    println!("user3 : {}", user3.username);

    const point1 :Point = Point(0,0,0);
    const color1 :Color = Color( 0, 0, 10);
    println!("point1 : {} , color: {}", point1.0, color1.0);
}


fn learn_variable(){
    println!("==============================================================================");
    let mut x = 5;
    const MAX_POINT :u32 = 100000;

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces len {}", spaces);

    println!("The value of x is {}", x);
    println!("max point value {}", MAX_POINT);
    x = 6;
    println!("The value of x is {}", x);
}

fn learn_datatype(){
    println!("==============================================================================");
    let guess :u32 = "42".parse().expect("Not a number !");
    println!("guess : {}", guess);

    let t :u32= 10 + 100;
    println!("t value : {} ", t);

    //declare tuple and destructuring
    let single_tuple : (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = single_tuple;
    println!("The value of x y z is : {} {} {}", x, y, z);
    println!("value tuple : {}", single_tuple.0);

    let months = ["January", "February", "Dkk"];
    let numbers: [i32;5] = [1,2,3,4,5];
    let mut index = 0;
    while index < numbers.len(){
        println!("value of numbers {} {}", index, numbers[index]);
        index = index + 1;
    }
    println!("first data : {}", numbers[0]);
    println!("first month : {}", months[0]);


//    let a = [1,2,3,4,5];
//    let index = 10;
//    let element = a[index];
//    println!("The value of element is : {}", element);
}

fn another_func(x: i32){
    println!("==============================================================================");
    println!("the value of x is {}", x);

    //expression
//    let x = 5;
    let y = {
        let x = 3;
        x+1
    };
    println!("value of y is {}", y);
    println!("add 1 + 2 : {}", add_func(1,2));
}

fn add_func(x: i32, y: i32) -> i32{
    x + y
}


fn try_control_flow(){
    println!("==============================================================================");
    let number = 6;
    if number !=0 {
        println!("condition was true");
    }else{
        println!("condition was false");
    }

    if number%4 == 0{
        println!("number is divisible by 4");
    }else if number%3 == 0{
        println!("number is divisible by 3");
    }else if number%2 == 0{
        println!("number is divisible by 2");
    }else{
        println!("number is not divisible by 4, 3, or 2");
    }

    let inline_if_value = if 10 > 3 {
        true
    }else{
        false
    };

    println!("inline if value : {}", inline_if_value);


    //loop, while, for
    //try loop
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10{
            break counter *2 ;
        }
    };

    println!("result {}", result);
    assert_eq!(result, 20);

    let a = [10, 20, 30, 40, 50, 60, 70];
    for element in a.iter(){
        println!("the value is : {}", element);
    }

    //this using range
    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn ownership_in_rust(){
    println!("==============================================================================");
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("string value {}",s);

    //value from string1 is moved into string2
    let string1 = String::from("hello");
    let string2 = string1;

    println!("{}, world", string2);


    //do deep copy
    let string3 = string2.clone();
    println!("string2 {}, string3 {}", string2, string3);

    //take ownership
    println!("before take ownership string2 value {}", string2);
    takes_ownership(string2);

    let x = 5;
    makes_copy(x);


    //takes ownership second
    let s1 = gives_ownership();

    let s2 = String::from("hello");
    let s3 = takes_and_give_back(s2);

    println!("s1 value : {}", s1);
    println!("s3 value : {}", s3);

    let string1_temp = String::from("hello");
    let (string2_temp, len) = calculate_length(string1_temp);
    println!("The Length of \"{}\" is {}", string2_temp, len);
}

fn reference_and_borrowing(){
    println!("==============================================================================");
    let s1 = String::from("hello");
    let len = calculate_length_with_dereferencing(&s1);
    println!("The Length of \"{}\" is {}", s1, len);

    let mut s = String::from("hello");
    change_string_value(&mut s);
    println!("value of : {}",s);

//    test_dangling_pointer();  //THIS CAUSE ERROR !

    //slice type
    let first_word_value = first_word(&String::from("another day of paradise"));
    println!("first word value {}", first_word_value);

    let s = String::from("hello, world");
    second_word(&s);


    //slices
    let a = [1,2,3,4,5,6];
    let slice_int1 = &a[1..3];
    for (_i, &value) in slice_int1.iter().enumerate(){
        println!("slice value : {}",&value);
    }
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }

    s.len()
}

fn second_word(s: &String) -> (usize, usize) {
    let hello = &s[0..=4];
    let world = &s[6..=10];
    return (hello.len(), world.len());
}

//test dangling pointer, THIS CAUSE ERROR !!!
//fn test_dangling_pointer() -> &String{
//    let s = String::from("hello");
//    &s;
//}



//helper functions
fn create_user(email: String, username: String) -> User{
    User{
        username,
        email,
        active: true,
        sign_in_count: 1
    }
}

fn change_string_value(s:&mut String){
    s.push_str(", world !");
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn gives_ownership() -> String{
    let some_string = String::from("hello");
    some_string
}

fn takes_and_give_back(a_string: String) -> String{
    a_string
}


fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}

fn calculate_length_with_dereferencing(s: &String) -> usize{
    s.len()
}
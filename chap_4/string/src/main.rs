fn main() {
    let mut s = String::from("hello");
    println!("{}", s); // This will print `hello'
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
    let s1 = s; // SHALLOW copy only stack data is copied. variable is moved to s1 form s and This will print `hello, world!`
    println!("{}", s1);
    // println!("{}", s); it will not work as variable moved

    let s3 = String::from("han gi");
    let mut s4 = s3.clone();  // DEEP copy of data both in stack and heap
   println!("s3 {}, and s4 {}",s3,s4);
   s4.push_str(" nawa aya hai ");
   println!("s3 after using clone {}, and updated s4 {}",s3,s4);
    // s3.drop;
let mut st = String::from("hello!!!!!!!");  // st comes into scope
    takes_ownership(&mut st); // st's value moves into the function...
                        // ... and so is no longer valid here
    let xy = 5;                      // x comes into scope
    makes_copy(xy);     // x would move into the function,
                        // but i32 is Copy, so it’s okay to still
    println!("printing xy int32 varible after moving it to function shows it can work {}", xy);// use x afterward
    let mut ss1 = String::from("calculate length ");

    println!("Original value of ss1 is :{}", ss1);
    let (len) = calculate_length(&mut ss1);
    println!("the value of ss1 is changed to :{}", ss1);
    println!("The length of '{}' is {}.", ss1, len);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
fn takes_ownership(some_string :&mut String) { // some_string comes into scope
    some_string.push_str(" adding data in function thorugh mutablility of string");
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
fn calculate_length(slen: &mut String) -> (usize) {
   slen.push_str("this is now added");
    let length = slen.len(); // len() returns the length of a String
    length
}
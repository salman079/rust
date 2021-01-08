use std::fmt::Display;
use std::cmp::PartialOrd;

fn main() {
pass(123);
pass(203984);
let abc = String::from("abcdef12345");
strpass(abc);
multiplepass(0987);
multiplepass("ASDFSADF");
//
let v1= vec![1,2,3,4,5,4,3,4,1];
highvec(&v1);
let v2= vec![4,3,4,5,8,5,4,8,6,7,9,7,5,4,8,9,6,5,4,3,4,1];
highvec(&v2);
let v3= vec!['a','d','f','g','l','j','t','h','s','g','e','r','y','t','q','w'];
strhighvec(&v3);
multihighvec(&v3);
multihighvec(&v1);
multihighvec(&v2);
}
fn pass(x: i32){
    println!("Integer Password: {}",x)
}
fn strpass(x: String){
    println!("String Password: {}",x)
}
fn multiplepass<T: Display>(x: T){
    println!("Multiple Password using Generic : {}",x)
}
fn highvec(x :&[i32]){
    let mut highest=x[0];
    for &item in x.iter(){
        if item > highest{
            highest = item;
        }
    }
    println!("Highest number in vector is {}",highest);
}
fn strhighvec(x :&[char]){
    let mut highest=x[0];
    for &item in x.iter(){
        if item > highest{
            highest = item;
        }
    }
    println!("Highest string in vector is {}",highest);
}
fn multihighvec<T: Display + PartialOrd+ Copy>(x: &[T]){
    let mut highest=x[0];
    for &item in x.iter(){
        if item > highest{
            highest = item;
        }
    }
    println!("Highest string in vector is {}",highest);
}
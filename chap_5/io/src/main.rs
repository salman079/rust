use std::io;

fn main() {
    println!("Enter name :!");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("panic error");
   
    println!("what is age: ");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("wrong input");
    let age_integer:u32 = age.trim().parse().expect("age should integer");
   
    println!("your name is : {} " , name);
    println!("age after 5 years {}: ",age_integer+5);
}

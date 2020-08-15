fn main() {
    let mut x = String::from("Hello, world!");
    let length = change(&mut x);
    println!("print in the fn main {}",x);
    println!("the length of {} is {}",x , length);

    let a = dangle();
    println!("{}",a);
}
fn change(s: &mut String) -> usize {
println!("print in the change funchtion {}",s);
s.push_str(" this is the addition in fn change");
println!("printing in the change funchtion after change {}",s);
s.len()
}
fn dangle() -> String{
let p=String::from("dangle");
p
}


fn main() {
    println!("Shadowing the value of a variable with Mutabiliting it !!!!!");
    let x = 8;
    println!("value of x {} ",x);
    let x = x + 5;
    println!("value of x {} ",x);
    let x = x * 2;
    println!("value of x {} ",x);

    println!("Shadowing - change datatype !!!!!");
    let y = "           ";
    let y =y.len();
    println!("value of y {} ",y);
    
    
    // the following code will not work using MUT
    let mut z = "    ";
  // z =z.len();
    println!("value of z {} ",z);
    
    println!("Another shadowing example");
    let a = 10; //  in variables annotation is not compulsory
    println!("value of 'a' {}", a);
    let a = a + 5; // shadowing
    println!("shaddowing value of 'a' by adding 5 without muttablity {}", a);
    let a:f64= 9.9232; // shadowing to float
    println!("shadowing 'a' to float {}", a);

}


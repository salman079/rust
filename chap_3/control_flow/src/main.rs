fn main() {
    println!("Control Flow");
    let number = 0;
    if number ==0 {
        println!("The number is 0");
    }
    else if number > 0 {
        println!("The number is positive");
    }
    else  {
        println!("The number is negaive ");
    }
let a = false;
let b = {
    if a {7}  // expresion
    else {8}  // expression
}; //Statement
println!("the value of a {}",a);
println!("the value of b {}",b);
}

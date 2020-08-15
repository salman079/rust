fn main() {
    println!("Hello, world!");
    second_function();
    passing_variables_function(2, 5, 6.9, 'P'); // within brackets we are passing argume                                                                                                                                                                                                                                                                                                                                                                                                                                          nts
       first_fun(2, 5);
    
    let multiply = multiply_fun(2, 5);
    println!("second function multiply, multiplication is done in second funcation whlle we pass 2 and 5 = {}",multiply);
    println!("second function multiply, multiplication is done in second funcation whlle we pass 10 and 15 = {}",multiply_fun(10, 15));

    // let tupple_square = tuple_fun(3, 4.5);  we can use this too
    // println!("using tupple squre function square 1: {},square 2: {}",tupple_square.0,tupple_square.1);
    let (o,p) = tuple_fun(3,4.5);  // we can use this too
    println!("using tupple squre function square 1: {},square 2: {}",o,p);
}
fn second_function(){
    println!("this is second function");
}
fn passing_variables_function(a:u32, x:u32, y:f32, z:char) { //passing_variables_function() is the function signature
    println!("passing_variables_function which is called from main fn and passed the value to be prinited the value of a= {}, x= {},y {} & z {}",a,x,y,z);
}
fn first_fun(a:u32, x:u32){  // within () bracket it is function parameters
println!("first function addition = {}",x+a); // within {} function defination
}
fn multiply_fun(a:u32, x:u32) -> u32{
   a*x   // this is expression 
}
fn tuple_fun(a:u32,b:f32) -> (u32, f32){
let square_1 = a*a;
let square_2 = b*b;
(square_1,square_2)
}
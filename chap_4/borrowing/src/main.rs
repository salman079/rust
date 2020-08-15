fn main() {
    println!("Hello, world!");

    let mut a:u8 = 10;
    let b = &a;
    let c = &b;
   
println!("the value of variable a is {}, and its memenory address is {:p}",a,b);
println!("the value of variable b is {}, and its memenory address is {:p}",b,c);
println!("the value of variable c is {}, and its memenory address is not saved in any variable ",c);


let x = &mut a;
// x.push_str("  Pakistan zindabad");

}
fn main() {
    println!("Compound Variables - TUPLES and ARRAY");
    // Tupple
    let tup: (i32,f64,u8) = (500,6.4,1);
    println!("Tupple {:#?}", tup);
    println!("Tupple - Accessing value by index 1 {}", tup.1);
    println!("destruction of a tupple");
    let (x,y,z)=tup;
    println!("x: {},y: {},z: {}",x,y,z);
    
    // Array 
    let arr: [u32;5] = [3;5];
    println!("Array {:#?}", arr);
    println!("Array - Accessing value by index 1 {}", arr[1]);
    println!("destruction of a Array");
    let [a,b,c,d,e]=arr;
    println!("a: {},b: {},c: {},d: {},e: {}",a,b,c,d,e);

}

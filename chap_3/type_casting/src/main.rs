fn main() {
// this is type casting
println!("This is type castig");
let a :u8 = 40;
let b :f32 = 20.23;
let c = a + b as u8;
println!("a:u8 {} + b:f32 as u8 {} = {}",a,b,c);
let c1 = a as f32 + b;
println!("a:u8 as f32 {} + b:f32 {} = {}",a,b,c1);
}

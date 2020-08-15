fn main() {
    
    for a in (0..5).rev(){
    println!("{} Hello, world!",a);
    }
    println!("===================");
    let array_lot = [3,5,4,0,44,-90,32];
    for b in (0..array_lot.len()){
    println!("array index {} value of array {} ",b,array_lot[b]);
    }
    println!("===================");
    let mut c =0;
    let a_lot = [13,35,44,20,2344,-9230,66432];
    for b in a_lot.iter() {
    println!("array index {} value of array {} ",c,b);
    c+=1;
    }
}

fn increment(num:Option<i32>) -> Option<i32>{
    match num {
        None => None,
        Some(i) => Some(i+1)
    }
}
fn main() {
    let seven = Option::Some(7);
    println!("{:?}",seven);
    let result = increment(seven);
    println!("{:?}",result);

    let nothing:Option<i32> = Option::None;
    println!("{:?}", nothing);

    
}
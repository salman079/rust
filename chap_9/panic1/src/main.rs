fn main() {
    println!("Hello, world!");
    let mut check = String::new();
    std::io::stdin().read_line(&mut check).expect("Error");
    let check = match check.trim().parse() {
        Ok(n) => {
            println!("Welcome");
            n
        },
        Err(_) => {panic!("Error after Hello World");0}
    
    };
    if check >= 10 {
        panic!("panic more than 10 in IF");
    }
    println!("After Panic {} ", check);
    
}

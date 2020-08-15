use std::io;

fn main() {
    println!("Enter your age: ");
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Error");
    println!("before match {:?}",data.trim());
    // let age :u8= data.trim().parse().unwrap();
    let age :u8= match data.trim().parse() {
        Ok(num) => {println!("in match {}",num);
                    num},
        Err(eee) => {println!("error in match {:?}",eee);
                    0 }
    };
    println!("after match {:?}",age);

    loop {
    println!("Enter your salary: ");
    let mut mysalary = String::new();
    io::stdin().read_line(&mut mysalary).expect("Error");
    let salary :u32 = match mysalary.trim().parse() {
        Ok(num) => {println!("valid input");
                    num},
        Err(eee) => {println!("not allowed, only digits");
                    continue}
    };
    break
    }
}
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt");
        let f = match f {
        Ok(file) => file, //println!("file pehlay say banee hai is lya mil gaee hai "),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(f) => f,
                Err(error) => panic!("file nahee ban rahee hai babu"),
            }
            _ => {
                panic!("bhai file nahee istamal ker saktay");
            }
        }  
    };
}
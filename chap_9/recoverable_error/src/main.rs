use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    
    let o = match f {
        Result::Ok(T) => println!("file mil gaee hai "),
        Result::Err(E) => {
            panic!("File nahee milee");
        },
    };
}

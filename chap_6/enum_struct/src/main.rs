#[derive(Debug)]
enum Foode {
    roti(u8),  //roti(String),
    pizza(String),  //piza(String),
    desi(String),    //desi(String),
}
#[derive(Debug)]
struct Foods {
    roti :String,
    pizza :String,
    desi :String,
}
impl Foode {
    fn ppt(&self) {
        println!("impl block of enum {:?}",self);
    }
}
impl Foods {
    fn ppt(&self) {
        println!("impl block of struct {:#?}",self);
    }
}
fn bill(e1:&Foode){
    match e1 {  // match function enum
        Foode::roti(number) => {println!("You purchase {:?} rotis ",number);
                                println!("your bill is {} rupees",number*10)},
        Foode::pizza(mypizza) => println!("your purchased  {} and your bill is 800 rupees",mypizza),
        Foode::desi(mydesi) => println!("your purchased  {} and your bill is 1800 rupees",mydesi),
        //_ => println!("your bill is 0 rupees",),
    };
}
fn main() {
    let e1 = Foode::roti(4);
    println!("e1 = {:?}",e1);
    e1.ppt();
    bill(&e1);
    let e2 = Foode::pizza(String::from("Chicken Fajita"));
    let e3 = Foode::desi(String::from("Chicken Mandi"));
    bill(&e2);
    bill(&e3);
    // match e1 {  // match enum
    //     Foode::roti(number) => {println!("You purchase {:?} rotis ",number);
    //                             println!("your bill is {} rupees",number*10)},
    //     Foode::pizza(mypizza) => println!("your purchased  {} and your bill is 800 rupees",mypizza),
    //     Foode::desi(mydesi) => println!("your purchased  {} and your bill is 1800 rupees",mydesi),
    //     //_ => println!("your bill is 0 rupees",),
    // }

    let s1 = Foods {
        roti : String::from("Garlic naan"),
        pizza : String::from("Chicken"),
        desi : String::from("Nehari")
};
    println!("s1 = {:#?}",s1);
    s1.ppt();
    let number = 3;
    println!("{}",number);
    let snumber = Some(88);  //let snumber = Optionn::Some(88);
    println!("{:?}",snumber);
    let sstring = Some(String::from("ice cream"));  //let snumber = Optionn::Some();
    println!("{:?}",sstring); 

    match number {  // match number 
        1 => println!("First postion"),
        2 => println!("Second postion"),
        3 => println!("Third postion"),
        _ => println!("{} not in the Top 3", number),
    };
    
    match snumber {  // match Option
        Some(88) => println!("you have {:?} rupees", snumber),
        None => println!("You have nothing"),
        Some(xyz) => println!("you have {}",xyz),
    };
}
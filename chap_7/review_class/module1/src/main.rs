mod finance {    
    pub fn salary(data:u32) {
        println!("Your salary is transfered to your Bank amounting to Rs.{}",data);
    }
}    
mod hr {
    pub mod imran {
        pub mod shahzad {
            pub fn recruit() {
                println!{"There is a new vacancy"};
            }
        }
    }
}
mod sales {
    pub fn today_sale() {
        println!{"Todays sales is Rs. 50,000,000"}
    }
}
mod marketing {
    pub fn display() {
        println!{"we have to display a banner @ National Statdium"}
    }
}
use crate::hr::imran::shahzad;  // alias  absolute path

fn main() {
    println!("Hello, world!");
    finance::salary(30000);// relative path
    hr::imran::shahzad::recruit(); // relative path
    crate::hr::imran::shahzad::recruit(); // absolute path
    sales::today_sale();// relative path
    marketing::display();   // relative path
    shahzad::recruit()
}

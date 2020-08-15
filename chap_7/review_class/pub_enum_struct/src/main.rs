mod irfan {
    #[derive(Debug)]
    pub enum Fruit {
        Watermelon,
        Cherries,
        Jamun,
        Falsay
    }
    #[derive(Debug)]
    pub struct Guest {
        pub name:String,
        pub age:u8,
    }
}
fn main() {
    println!("Hello, world!");
    let myfruit = irfan::Fruit::Jamun;
    println!("My favorite fruit {:?}", myfruit);
    let myguest = irfan::Guest {
        name : String::from("Irfan Haider"),
        age : 51
    };
    println!("Guest {:?}", myguest);
}
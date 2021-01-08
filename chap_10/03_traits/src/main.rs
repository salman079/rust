#[derive(Debug)]
struct Superman {
    name: String,
}
#[derive(Debug)]
struct Batman {
    name: String,
}
#[derive(Debug)]
struct Hulk {
    name: String,
}
#[derive(Debug)]
struct Spiderman {
    name: String,
}
pub trait Power {
    fn power_score(&self) -> u8 {
        //default implimentation
        50
    }
}
impl Power for Superman {
    fn power_score(&self) -> u8 {
        100
    }
}
impl Power for Batman {
    fn power_score(&self) -> u8 {
        80
    }
}
impl Power for Hulk {}
impl Power for Spiderman {}

fn main() {
    let my_superman = Superman {
        name: String::from("Superman"),
    };
    let my_batman = Batman {
        name: String::from("Batman"),
    };
    let my_hulk = Hulk {
        name: String::from("Hulk"),
    };
    let my_spiderman = Spiderman {
        name: String::from("Spiderman"),
    };
    println!("My Charaters and their powers ");
    println!("{:?} has power of {:?}", my_superman,my_superman.power_score());
    println!("{:?} has power of {:?}", my_batman, my_batman.power_score());
    println!("{:?} has power of {:?}", my_hulk, my_hulk.power_score());
    println!("{:?} has power of {:?}", my_spiderman,my_spiderman.power_score());
}

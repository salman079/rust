#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}
struct Point1<T>{
    xx:T,
    yy:T,
}
impl <T> Point1 <T>{
    fn area(&self)-> &T{
        &self.xx
    }
}
impl Point1<f64>{
    fn area1(self)->f64{
        self.xx*self.yy
    }
}
fn main() {
    let integer = Point { x: 10, y: 11 };
    let string = Point {
        x: String::from("salman"),
        y: String::from("aman"),
    };
    let int_string = Point {
        x: 10,
        y: String::from("Tooba"),
    };
    let integer1 = Point1 { xx: 100, yy: 110 };
    let float1 = Point1 { xx: 1.4, yy: 2.5 };
    println!("This is integer {:?}", integer);
    println!("This is string {:?}", string);
    println!("This is integer and string {:?}", int_string);
    println!("This is Point1 calling {:?}",integer1.area());
    println!("This is Point1 float {:?}",float1.area1())

}
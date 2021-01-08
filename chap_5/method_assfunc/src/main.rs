#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// using function
fn area(a: &Rectangle) -> u32 {
    a.height * a.width
}
// using imp block
impl Rectangle {
    //one parameter
    fn area_1(&self) -> u32 {
        self.height * self.width
    }
    //more parameter
    fn is_big(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
    //Associated function it returns the datatype
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect_1 = Rectangle {
        width: 100,
        height: 90,
    };
    let rect_2 = Rectangle {
        width: 80,
        height: 70,
    };
    let rect_3 = Rectangle {
        width: 85,
        height: 70,
    };
    // calling function
    println!("Rectangle 1 {:#?}", rect_1);
    println!("Area of Rectangle 1 using function: {}", area(&rect_1));
    println!("Area of Rectangle 2 using function: {}", area(&rect_2));
    println!("Area of Rectangle 3 using function: {}", area(&rect_3));
    // calling impl block ie Method
    println!("Area of Rectangle 1 using impl block: {}", &rect_1.area_1());
    println!("Area of Rectangle 2 using impl block: {}", &rect_2.area_1());
    println!("Area of Rectangle 3 using impl block: {}", &rect_3.area_1());
    println!("{:?}", rect_3);
    // calling impl block ie Method
    println!("Is rect_1 greater than rect_2 {}", &rect_1.is_big(&rect_2));
    println!("Is rect_2 greater than rect_3 {}", &rect_2.is_big(&rect_3));
    // calling associated function
    println!("calling of associated function {:#?}", Rectangle::square(9));
}

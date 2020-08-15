struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("1 -using variable - The area of the rectangle is {} square pixels.",area1(width1, height1));
    
    let rect1 = (30, 50);
    println!("2 - using Tuple - The area of the rectangle is {} square pixels.",area2(rect1));

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("3 - using struct - The area of the rectangle is {} square pixels.",area3(&rect2));

}
fn area1(width: u32, height: u32) -> u32 {
    width * height
}
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area3(rect3: &Rectangle) -> u32 {
    rect3.width * rect3.height
}
#[derive(Debug)]
struct Book {
    name: String,
    author: String,
    price: u16,
    availibility: bool,
}
fn main() {
    let mut book1 = Book {
        name:String::from("ABC book"),
        author:String::from("Papo"),
        price: 200,
        availibility: true,
    };
    let book2 = Book {
        price: 900,
        availibility: false,
        name:String::from("XYZ book"),
        author:String::from("Lappapo"),
    };
    let book44 = Book {
        price: 900,
        availibility: false,
        name:String::from("XYZ book"),
        author:String::from("Lappapo"),
    };
    let book4 = Book {
        name:String::from("book 4"),
        ..book44
    };
println!("Book 1 {:?}",book1);
let x=book1.name;
book1.name = String::from("ABCDEFG Book");
println!("after changing name of the book 1 from {} to {}",x,book1.name);
println!("Book 1 {:?}",book1);
book22(book2);
println!("calling book3 from function {:?}",book3());
println!("book 4 similar to book 44 except for name {:?}", book4);
}
fn book3 () -> Book {
    Book {
        name:String::from("book in book3 function"),
        author:String::from("book 3 author"),
        price: 1830,
        availibility: true,
    }
}
fn book22 (z:Book) {
println!("prinitng Book 2 in function calling from fn() {:?}", z);
}
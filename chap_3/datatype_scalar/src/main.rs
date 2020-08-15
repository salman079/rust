const PI:f64 = 3.164; // contant. In constant annotation is madatory :f64 is annotation. Constant should be in caps
fn main() {
    println!("Hello, world!");
    let mut temprature:u32 = 35; // mutable variable
    println!("Today's temprature is {}", temprature);
    temprature = 25; 
    println!("Mutable temprature value now changed {}", temprature);
println!("+++++++++++++++++++++++++++++++++++");
    println!("PI value = {}", PI);   
    let x = 10; //  in variables annotation is not compulsory
    println!("{}", x);
println!("+++++++++++++++++++++++++++++++++++");
    let mut condition: bool = true;
    println!("boolean condition :{}", condition);
    condition = false;
    println!("updated boolean condition :{}", condition);
println!("+++++++++++++++++++++++++++++++++++");
    let char_1: char = 'a';  // this is character no need to mention char
    println!("character {}",char_1);
    let name: &str = "salman";   // this is String literal no need of &str
    println!("string literal {}",name);
 println!("+++++++++++++++++++++++++++++++++++");   
    let name1:char = 'S'; // this is character type with one letter and single quote 
    let full_name = "Salman"; // this is string literal
    println!("this is string literal {} and this is character one letter only and single quotation: {}", full_name,name1);
println!("+++++++++++++++++++++++++++++++++++");
    let decimalnumber = 100; // decimal number
    let hexadecimalnumber = 0xff; // hexa decimal
    let octanumber = 0o77; // octa number
    let binarynnumber = 0b1010001010; // binary numbers
    let bytenumber = b'A';
println!("decimalnumber {}", decimalnumber);
println!("hexadecimalnumber {}", hexadecimalnumber);
println!("octanumber {}", octanumber);
println!("binarynnumber {}", binarynnumber);
println!("bytenumber {}", bytenumber);
}
fn main() {
    // initializing string
    let s = String::new();
    let s1 = String::from("hello world");
    println!("s: {} and s1: {}",s,s1);

    // convert from string literal to String type
    let s3 = "hello world";
    let s4 = s3.to_string();
    let s5 = "pakistan".to_string();
    println!("s3: {} and s4: {} and direct conversion s5: {}",s3,s4,s5);

    // appending the string
    let mut s6 = String::from("Hello ");
    s6.push_str("world ");
    println!("s6: {}",s6);
    
    let s7= "Pakistan";
    s6.push_str(s7);
    println!("s7: {}",s6);

    // pushing char to String
    let mut ss = String::from("Appl");
    ss.push('e');
    println!("Adding char to a string: '{}'",ss);
    
    // concatenating strings type
    let s8 = String::from("Tooba");
    let s9 = String::from("Salman");
    let s10 = String::from("Siddiqui");  
    let s11 = "-"; // string literal
    let s12= s8+s11+&s9+s11+&s10; // & to use for String type not for String literal 
    println!("Sum of string type and string literals: s8+s9+s10+s11: {}",s12);
    // concatenating with Format Macro
    let s13 = format!("-{}-{}",s9,s10);
    println!("prining using format funtion {}",s13);

    // indexing in str
    println!("indexing of {} {}",s7,&s7[0..2]);
  
    // indexing in str using iteration
    for c in s7.chars(){
        println!("indexing using chars funcation on {} '{}'",s7, c);
    }
    // iteration on String using bytes()
    for c in s7.bytes(){
    println!("indexing using chars funcation on {} '{}'",s7, c);
    }
}

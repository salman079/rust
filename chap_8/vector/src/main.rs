
fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = Vec::new(); // creating Empty Vector
    let mut v1 = vec![10,20,30,40];  // creating Vector with value
    println!("v {:?} and v1 {:?}",v,v1);

    let m = 5;
    v.push(m); // push at the end of the vector
    v1.push(50);  // push at the end of the vector
    println!("values after pushing v {:?} and v1 {:?}",v,v1);
    
    v.pop(); // poping the values
    v1.pop(); // poping the values
    println!("values after poping v {:?} v1 {:?}",v,v1);

    println!("Accessing Vector index: V1  1st {}, 2nd {}, 3rd {}, and 4th {} value",v1[0],v1[1],v1[2],v1[3]);
    let element1 = v1[1];
    let element2 = &v1[1];
    let element3 = v1.get(9);
    println!("accessing element via variable: {}",element1);
    println!("accessing element via reference: {}",element2);
    println!("accessing element via get: {:?}",element3);
    match element3 {
        Some(i) => println!("accessing element via get-match-option: {:?}",i),
        None =>    println!("No element is excess ")
    };
    
    for i in &v1 { // we use & for reference because to pass the referece else we will have a Move/borrowoing erro
        println!("for loop {:?}",i);
    }
    println!("printing vector after loop scope finishes {:?}",v1);
    
    for k in &mut v1 { // we use & for reference because to pass the referece else we will have a Move/borrowoing erro
        *k += 100;
        println!("for loop {:?}",k);
    }
    println!("printing vector after loop scope finishes {:?}",v1);
    
    for j in v1 { // we use & for reference because to pass the referece else we will have a Move/borrowoing erro
        v.push(j+5);
       }
    println!("new vector (v) after pushing in loop {:?} and we can now not able to use vector v1",v);
    
// using enum to record multiple type of variable in a vector    
let row = vec![SpreadSheetCell::int(50),
                SpreadSheetCell::flot(10.5),
                SpreadSheetCell::text(String::from("Hello"))];
println!("{:?}",row);
}

#[derive(Debug)]
enum SpreadSheetCell {
    int(i32),
    flot(f64),
    text(String)
}
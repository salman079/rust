fn main() {
    // loop 1
    let mut counter=0;
    loop {
        counter += 1;
        println!("counter {}",counter);
        if counter == 5 {break};
    }
    println!("{}", counter);
    // new loop with let
    let mut counter2 =0;
    let loop_counter = loop{
        counter2=counter2+1;
        println!("counter_2 value is {}", counter2);
        if counter2 ==5 {break counter2*2}
    };
    println!("loop_counter {} and its value is {} ",counter2,loop_counter);
// loop in array
let mut c_array = 0;
let array_no = [3,9,20,23,58,34];
loop {
    println!("Array index {} value is {}", c_array,array_no[c_array]);
    c_array=c_array +1;
    if c_array == array_no.len() {break};
}
}

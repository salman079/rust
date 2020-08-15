#[derive(Debug)]
enum Move {
    Walk,
    Jump,
    Hop,
    Run,
}
#[derive(Debug)]
enum Sweet {
    Choclate(String),
    Sweet(String),
    HomeSweet(String),
    Open(String,u32),
}
// method deffination
impl Move {
    fn ppt(&self){
        println!("Implemention block method: move {:?}",self);
    }
}
// function defination
fn daily(data:Move){
    println!("Kid move of data from main {:?}",data);

    match data {
        Move::Walk => {println!("walk is necessary ");
                        println!("Daily 40 minns");},
        Move::Jump => println!("You are burning more calories"),
        Move::Hop => println!("Good calories"),
        Move::Run => println!("Most calorie"),
    }
}
fn main (){
    // using Move enum
    let mymove = Move::Walk;
    println!("My move: {:?}",mymove);
    
    let urmove = Move::Run;
    println!("Your move: {:?}",urmove);
    
    let kidmove = Move::Jump;
    daily(kidmove);  // function calling
    daily(mymove);   // function calling
    daily(urmove);   // function calling
    
    let babymove = Move::Hop;
    babymove.ppt();  // method calling

    // using Sweet enum
    let mysweet = Sweet::Choclate(String::from("Rocker"));
    let ursweet = Sweet::Open(String::from("chicky"),1);
    println!("My sweet {:?}",mysweet);
    println!("Your sweet {:?}",ursweet);
}
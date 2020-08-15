use std::collections::HashMap;

fn main() {
    // initialise HashMap
    let mut map = HashMap::new();
    
    // inserting values in Hashmaps
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Yellow"), 50);
    println!("{:?}",map);
    
    // accessing value in HashMap
    println!("Accessing value using key which is Yellow: {:?}",map.get("Yellow"));
    println!("Accessing value using key which is ellow: {:?}",map.get("ellow"));

    // accessing value in HashMap throug Loop
    for (key,value) in &map{
        println!("Accessing through loop {},{}",key,value);
    }
    // Overwriting keys in HashMap
    map.insert(String::from("Yellow"), 500);
    println!("Yellow value is now updated {:?}",map);
    
    // Check dublication in relation to keys in HashMap
    map.entry(String::from("Purple")).or_insert(350);
    map.entry(String::from("Yellow")).or_insert(400);
    println!("using entry /or insert Purple and Yellow {:?}",map);
    
    //constructing HashMap using collect () method using Vector
    let teams = vec![String::from("Red"), String::from("Green")];
    let scores = vec![100,500];
    let map2: HashMap<_,_> = teams.iter().zip(scores.iter()).collect();
    println!("HashMap using Vector {:?}",map2);
    println!("{:?},{:?}",teams, scores);

    //constructing HashMap using collect () method using Array
    let teams1 = [String::from("Red-1"), String::from("Green-1")];
    let scores1 = vec![1100,1500];
    let map1: HashMap<_,_> = teams1.iter().zip(scores1.iter()).collect();
    println!("Hashmap using Array {:?}",map1);
    println!("{:?},{:?}",teams1, scores1);

    // ownership of variables in Hashmap
    let a = String::from("Fav Color");
    let b = String::from("Blue");
    let mut color = HashMap::new();
    color.insert(&a,&b);   // Ownership of String is tranfered to color because it saves in Heap so we cant use these variables again
    println!("{:?}",color);
    println!("{:?}",a); //we cant use variable a and b here as it ownership is transfered therefore we only useed reference
    println!("{:?}",b);
    
    let aa = "Fav Food";
    let bb = "Pizza";
    let mut food = HashMap::new();
    food.insert(aa,bb);   // Ownership of variables other than saved in Heap does not tranfers
    println!("{:?}",food);
    println!("{:?}",aa);
    println!("{:?}",bb);   
}

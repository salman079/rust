// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
#![allow(dead_code)]  // to avoid warnings of not using functions
mod front_house {  // mod = module
    pub mod hosting {
        pub fn add_towaitlist() {

        }
    }

    mod server {
        fn take_order() { 

        }
        fn order_serve() {

        }
        fn payment() {

        }

    }
}
fn eat_at_resturant() {
    //Absolute path
    crate::front_house::hosting::add_towaitlist();
    //Relative path
    front_house::hosting::add_towaitlist();
 
}
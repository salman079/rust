mod food;
mod lunch;
use lunch::serving;
use lunch::serving as ml;
fn main() {
    println!("Hello, world!");
    serving::karhi_chawal();
    food::sweets();
    lunch::serving::chana_chawal();
    ml::chana_chawal();
}
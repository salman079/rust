#[derive(Debug)]
struct Tweet {
    username:String,
    content:String
}
#[derive(Debug)]
struct NewsArticle {
    author:String,
    content:String
}
trait Summary{
    fn summarize_author(&self)->String;
    fn summarize(&self)->String{
        format!("{}",self.summarize_author())
    }
}
impl Summary for Tweet{
    fn summarize_author(&self)->String{
        format!("{}",self.content)
    }
}
impl Summary for NewsArticle{
    fn summarize_author(&self)->String{
        format!("{}",self.content)
    }
}
fn main() {
    let tweet1=Tweet{
        username:String::from("IK"),
        content:String::from("Chay")
    };
    let newsarticle1=NewsArticle{
        author:String::from("Bano Qudsia"),
        content:String::from("Raja Gidh")
    };
    println!("{}",tweet1.summarize());
    println!("{}",newsarticle1.summarize());
}
struct Tweet {
    username: String,
    content: String
}
struct NewsArticle {
    author: String,
    content: String
}
trait Summary {
    fn summarize_author(&self)->String;
    fn summarize(&self)->String{
        format!("{}",self.summarize_author())
    }
}
impl Summary for Tweet{
    fn summarize_author(&self)->String{
        format!("{}",self.username)
    }
}
impl Summary for NewsArticle{
    fn summarize_author(&self)->String{
        format!("{}",self.author)
    }
}
fn main() {
    let tweet1 = Tweet{
        username:String::from("Imran Khan"),
        content:String::from("IK is a PM")
    };
    let newsarticle1 = NewsArticle{
        author:String::from("Bano Qudsia"),
        content:String::from("Raja Qidh")
        };
    println!("{}",tweet1.summarize());
    println!("{}",newsarticle1.summarize());
}
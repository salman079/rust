struct NewsArticle {
    author: String,
    content: String,
}
struct Tweet {
    username: String,
    content: String,
}
pub trait Summary{
    fn summarize(&self)->String;
}
impl Summary for NewsArticle{
    fn summarize(&self)->String {
        format!("{} by {} ",self.author,self.content)
    }
}
impl Summary for Tweet{
    fn summarize(&self)->String{
        format!("{} by {} ",self.username,self.content)
    }
}
    fn main() {
    let _tweet_1 = Tweet {
        username: String::from("John"),
        content: String::from("Honesty is the best policy"),
    };
    let _newsarticle_1 = NewsArticle {
        author:String::from("Jeff"),
        content:String::from("Its rainging"),
    };
    println!("{}",_tweet_1.summarize());
    println!("{}",_newsarticle_1.summarize())
}

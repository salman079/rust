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
    fn summarize(&self)->String;
}
impl Summary for Tweet{
    fn summarize(&self)->String{
        format!("@{} posted this: {}",self.username,self.content)
    }
}
impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("{} is written by {}",self.author,self.content)
    }
}
fn notify(item1:impl Summary, item2:impl Summary)->String{
format!("{} and {} ",item1.summarize(),item2.summarize())
}
fn main() {
    let tweet1=Tweet{
        username:String::from("Salman"),
        content:String::from("Pakistan Zindabad")
    };
    let newsarticle1=NewsArticle{
        author:String::from("Smarter Way"),
        content:String::from("Python")
    };
    println!("Implimentation Traits - multiple typed of struct can be called");
    println!("{}",notify(tweet1, newsarticle1));
}

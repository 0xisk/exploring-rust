pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

fn main() {
    let tweet: Tweet = Tweet {
        username: String::from("@iskdrews"),
        content: String::from("Hi There!"),
        reply: false,
        retweet: false,
    };

    let article: NewsArticle = NewsArticle {
        author: String::from("Isk"),
        headline: String::from("Learning Rust!"),
        content: String::from("Happy to learn Rust!"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

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
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
}

pub struct Post {
    pub username: String,
    pub post: String,
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
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

    let post: Post = Post {
        username: String::from("Isk"),
        post: String::from("This is a dumb post"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
    println!("Post summary: {}", post.summarize());
}

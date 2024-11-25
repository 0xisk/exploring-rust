use core::fmt::Debug;
use std::fmt::Display;

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

pub trait Display {}

pub trait Clone {}

// Single traits
pub fn notifySingleTrait1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notifySingleTrait2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple traits
pub fn notifyMultipleTraits1(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
}

pub fn notifyMultipleTraits2<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
}

// T or impl are required to be both Summary and Display
pub fn notifyMultipleTraits3(item1: &(impl Summary + Display), item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
}

pub fn notifyMultipleTraits4<T: Summary + Display>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
}

// Becuase the types here are unreadable
// we can use the "where"
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    1
}

fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

// Conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is x = {}", self.y);
        }
    }
}

// Traits as return types
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("@iskdrews"),
        content: String::from("Hi There!"),
        reply: false,
        retweet: false,
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

    notifyMultipleTraits1(&article, &post);

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
    println!("Post summary: {}", post.summarize());

    println!("{}", returns_summarizable().summarize());
}

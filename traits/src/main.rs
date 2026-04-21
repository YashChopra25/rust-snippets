use std::{fmt::format, iter::Sum};

trait Summary {
    fn summarize(&self) -> String;
}
#[derive(Debug)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        let content = format!("Tweet by {}: {}", self.username, self.content);
        content
    }
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        let content = format!(
            "The news article author is:{}, and {} ",
            self.author, self.content,
        );
        content
    }
}
fn main() {
    println!("Hello, world!");
    let tweet = Tweet {
        username: String::from("YashChopra"),
        content: String::from("This news is very helpful"),
        reply: false,
        retweet: false,
    };
    let news = NewsArticle {
        headline: String::from("US vs IRAN wars"),
        location: String::from("strait of harmuz"),
        author: String::from("YashChopra25"),
        content: String::from("Serious war is going on"),
    };
    println!("Tweet: {:#?}\n News: {:#?}", tweet, news);
    news_aggregator(tweet);
    news_aggregator(news);
}
// fn news_aggregator(tweet:Tweet){
//     println!("There is a news in the market")
//     println!("The news is that:{}, it is published by {}", tweet.content, tweet.username)
// }

fn news_aggregator(tweet: impl Summary) {
    println!("There is a news in the market");
    println!("{}", tweet.summarize())
}

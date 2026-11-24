use aggregator::{NewsArticle, SocialPost, Summary};

fn main() {
    let news_a = NewsArticle {
        title: String::from("hello Rust"),
        author: String::from("Korea"),
        date: String::from("B.Jeong"),
        content: String::from("To be continued..."),
    };
    println!(
        "author: {}, summary: {}",
        news_a.get_author(),
        news_a.summarize()
    );
    let post_a = SocialPost {
        title: String::from("hi Rust"),
        author: String::from("USA"),
        date: String::from("C.Jeong"),
        content: String::from("Follow for more..."),
    };
    println!(
        "author: {}, summary: {}",
        post_a.get_author(),
        post_a.summarize()
    );
}
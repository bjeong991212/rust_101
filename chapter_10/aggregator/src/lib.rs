pub trait Summary {
    fn get_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("tbd by {}", self.get_author())
    }
}

pub struct NewsArticle {
    pub title: String,
    pub author: String,
    pub date: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn get_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct SocialPost {
    pub title: String,
    pub author: String,
    pub date: String,
    pub content: String,
}

impl Summary for SocialPost {
    fn get_author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{} by {} (date: {})", self.title, self.get_author(), self.date)
    }
}
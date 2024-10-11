pub trait Summary {
    fn summarize(&self) -> String;
    fn all(&self) -> String;
    fn read_more(&self) -> String {
        String::from("Read more...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn all(&self) -> String {
        format!(
            "{} by {} at {} :: {}",
            self.headline, self.author, self.location, self.content
        )
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
        format!("@{}: {}", self.username, self.content)
    }
    fn all(&self) -> String {
        format!(
            "@{}: {}. reply?: {}, retweet?: {}",
            self.username, self.content, self.reply, self.retweet
        )
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking News: {}", item.summarize());
}

// an alternative
/*
 * pub fn notify<T: Summary>(item: &T) {
 *  println!("Breaking News: {}", item.summarize());
 * }
 *
 */

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read More...)")
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
        format!("{}, by {} ({})", self.headline, self.author, self.location,)
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
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Task {
    pub heading: String,
    pub details: String,
}

impl Summary for Task {}

// without trait bound syntax - Easy to read
// pub fn notify(item: &impl Summary){
//     println!("Breaking news! {}", item.summarize());
// }

// Trait bound syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

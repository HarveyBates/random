// A trait is similiar to an interface in C++

// Define a Summary trait
// This includes the method signatures
pub trait Summary {
    // Can implemement some default
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implement trait on type
impl Summary for NewsArticle {
    // Or define it explicitly
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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

fn main() {
    let na = NewsArticle {
        headline: String::from("The headline"),
        content: String::from("This would contain some content"),
        author: String::from("Author name"),
        location: String::from("Location"),
    };
    println!("{}", na.summarize());
}

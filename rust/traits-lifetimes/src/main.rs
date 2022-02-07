
// A trait is similiar to an interface in C++

// Define a Summary trait
// This includes the method signatures
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implement trait on type
impl Summary for NewsArticle {
    fn summarize(&self) -> String{
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn main() {
    let na = NewsArticle {
        headline: String::from("The headline"),
        content: String::from(
            "This would contain some content",
        ),
        author: String::from("Author name"),
        location: String::from("Location"),
    };
    println!("{}", na.summarize());
}

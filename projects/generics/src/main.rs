// Calling another method within the same trait
// pub trait Summary {
//     fn summarize_author(&self) -> String;

//     fn summarize(&self) -> String {
//         format!("(Read more from {}...", self.summarize_author())
//     }
// }

// Defining the method signature and a default behavior
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Defining the method signature for custom Summary trait type
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub contenct: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait as a parameter with &impl shortcut
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// "Trait Bound" syntax, forcing both item1 and item2 to be of the same generic type that implements Summary
// pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// Specifying multiple Trait Bounds with the + Syntax
// pub fn notify<T: Summary + Display>(item: &T) {}

// Clearer Trait Bounds with `where` Clauses

// the long way
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// the `where` clause
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }

// Returning types that implement traits
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
}

// Using Trait Bounds to conditionally implement methods
use std::fmt::Display;

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
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {}

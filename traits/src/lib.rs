// a Summary trait that consists of the behavior provided by a summarize method
pub trait Summary {
    // fn summarize(&self) -> String;

    // definition of a summary trait with a default implementation of the summarize method
    // fn summarize(&self) -> String {
    //     String::from("(Read more...)")
    // }
    
    fn summarize_author(&self) -> String;

    // calling another method in the same trait
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// implementing the Summary trait on the NewsArticle and Tweet types
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// using the default implementation of summarize
impl Summary for NewsArticle {
    //fn summarize(&self) -> String {
    //    format!("{}, by {} ({})", self.headline, self.author, self.location)
    //}

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// traits as parameters
// function that accepts any type that implements Summary trait as parameter
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// the same but more verbose (trait bound syntax)
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

use std::fmt::{Display, Debug};

// specifying multiple trait bounds with the + syntax
pub fn notify3(item: &(impl Summary + Display)) {}
pub fn notify4<T: Summary + Display>(item: &T) {}

// clearer trait bounds with where clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {1}
fn some_function2<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{1}

// function that return a type that implements the Summary trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// Conditionally implement methods on a generic type depending on trait bounds
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

// conditionally implement a trait for any type that implements another trait
// impl<T: Display> ToString for T {
//     // --snip--
// }

// let s = 3.to_string();

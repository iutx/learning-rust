use std::fmt::{Debug, Display};

fn main() {
    let post = Post {
        title: String::from("Post Title"),
        author: String::from("Post Author"),
        content: String::from("Post Content"),
    };
    let post_2 = Post {
        title: String::from("Post Title 2"),
        author: String::from("Post Author 2"),
        content: String::from("Post Content 2"),
    };
    println!("post: {}", post.summarize());
    println!("post_2: {}", post_2.print());

    let news = News {
        writer: String::from("News Writer"),
        content: String::from("News Content"),
    };
    println!("news: {}", news.summarize());
    println!("news: {}", news.print());

    // Use trait as parameter
    trait_params(&post);
    trait_params(&news);

    // Trait bound
    // T must same type, and implement Summary trait
    // trait_bound(&post, &news);
    trait_bound(&post, &post);

    // Multi trait bound
    multi_trait_bound(&post);

    // Use where in trait bound
    where_trait_bound(&post, &news);

    // Trait bound for generic struct method
    // int、char、String already implement Display and PartialOrd
    let pair = Pair::new(1, 2);
    pair.cmp_display();
    // Post implement Display, but not implement PartialOrd, will cause compile error
    // let pari_summary = Pair::new(&post, &post_2);
    // pari_summary.cmp_display();

    // Trait return type
    // Must same type, and implement Summary trait
    trait_return_type(true);

    // derive trait
    // e.g. #[derive(Debug)], #[derive(PartialEq)], #[derive(Copy, Clone)]
    // #[derive(Debug)] will auto implement Debug trait for the struct
    // If not use derive, need to implement by self
}


// Similar to Interface. It is a way to define a set of methods that a type must implement.
pub trait Summary {
    fn summarize(&self) -> String;
    // Default implementation, can be override
    fn print(&self) -> String {
        format!("Read more from {}", self.summarize())
    }
}

pub struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("{} by {} {}", self.title, self.author, self.content)
    }
}

impl Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "impl display: ({}, {}, {})", self.title, self.author, self.content)
    }
}

pub struct News {
    writer: String,
    content: String,
}

impl Summary for News {
    fn summarize(&self) -> String {
        format!("{} by {}", self.content, self.writer)
    }
    // Override default implementation
    fn print(&self) -> String {
        format!("There push a news: {}", self.summarize())
    }
}

impl Debug for News {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "impl debug: ({}, {})", self.writer, self.content)
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Only implement for Pair<T> which implement Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


fn trait_params(item: &impl Summary) {
    println!("trait_params: {}", item.print());
}

// Trait bound
// More clear than trait_params: (item1: &impl Summary, item2: &impl Summary)
// Limit param type is T which implement trait Summary.
// T can be any type, but must same type.
fn trait_bound<T: Summary>(item1: &T, item2: &T) {
    println!("trait_bound: {}", item1.print());
    println!("trait_bound: {}", item2.print());
}

// Multi trait bound
fn multi_trait_bound<T: Summary + Display>(item: &T) {
    println!("multi trait bound, display: {}", item);
}

// Use where in trait bound
// Equal to `<T: Summary + Display, U: Summary + Debug>(t: &T, u: &U)`, more clear
fn where_trait_bound<T, U>(t: &T, u: &U)
    where T: Summary + Display,
          U: Summary + Debug
{
    println!("where_trait_bound: {}", t.print());
    println!("where_trait_bound: {:?}", u);
}

// Trait return type
// But it can only return one type, and it must be the same type.
fn trait_return_type(switch: bool) -> impl Summary {
    let mut post = Post {
        title: String::from("Post Title"),
        author: String::from("Post Author"),
        content: String::from("Post Content"),
    };
    let _ = News {
        writer: String::from("News Writer"),
        content: String::from("News Content"),
    };

    match switch {
        true => post,
        false => {
            println!("can't return multi type");
            // If return News, will cause compile error
            // news
            post.author = String::from("Post Author 2");
            post
        }
    }
}
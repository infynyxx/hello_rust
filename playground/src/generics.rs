#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

use std::fmt::Debug;
use std::fmt::Display;

struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> std::string::String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> std::string::String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "({} {})", self.username, self.content)
    }
}

fn notify<T: Summary + Display>(item: T) -> String {
    format!("Breaking news! {}", item.summarize())
}

fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Summary,
    U: Clone + Debug,
{
    1i32
}

fn get_summarizable(switch: bool) -> impl Summary {
    /**
    if (switch) {
        NewsArticle {
            headline: "Hello World".to_string(),
            location: "New York".to_string(),
            author: "Prajwal".to_string(),
            content: "This  is content".to_string(),
        }
    } else {
        Tweet {
            username: "infynyxx".to_string(),
            content: "Hello World!".to_string(),
            reply: false,
            retweet: false,
        }
    }**/
    Tweet {
        username: "infynyxx".to_string(),
        content: "Hello World!".to_string(),
        reply: false,
        retweet: false,
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[test]
fn test_distance_from_origin() {
    let point = Point { x: 22.1, y: 22.3 };
    assert_approx_eq!(point.distance_from_origin(), 31.39, 0.01f32);
}

#[test]
fn test_notify() {
    let tweet = Tweet {
        username: String::from("infynyxx"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };
    assert_eq!(notify(tweet), "Breaking news! infynyxx: Hello World!")
}

#[test]
fn test_largest() {
    assert_eq!(largest(&vec![22, 1, 300, -5, 66]), 300);
    assert_eq!(largest(&vec!['a', 'x', ' ', 'Z']), 'x');
    assert_eq!(largest(&vec![22.5, 33.66, 33.667]), 33.667);
}

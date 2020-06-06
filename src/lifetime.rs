// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

// omitted generics
fn print(s: &str) {
    println!("{}", s);
}

// expanded
fn print_expanded<'a>(s: &'a str) {
    println!("{}", s);
}

fn trim(s: &str) -> &str {
    s
}

fn trim_expanded<'a>(s: &'a str) -> &'a str {
    s
}

fn get_str() -> &'static str {
    "static"
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/**
 * The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations.
 * The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes.
 * If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes,
 * the compiler will stop with an error. These rules apply to fn definitions as well as impl blocks.
 *
 * The first rule is that each parameter that is a reference gets its own lifetime parameter.
 * In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
 * a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
 *
 * The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all
 * output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
 *
 * The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self
 * because this is a method, the lifetime of self is assigned to all output lifetime parameters.
 * This third rule makes methods much nicer to read and write because fewer symbols are necessary.
*/

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, s: &str) -> &str {
        println!("Announcement: {}", s);
        self.part
    }
}

fn longest_with_announcement<'a, T: std::fmt::Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn main() {
    print("omitted generics");
    print_expanded("expanded generics");
    trim("trim");
    trim_expanded("trim expanded");
    println!("{}", get_str());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        let str1 = "xyz";
        let str2 = "This is long one";
        assert_eq!(longest(&str1, &str2), str2);
    }

    #[test]
    fn test_longest_with() {
        let str1 = "xyz";
        let str2 = "This is long one";
        let announcement = "Something here";
        assert_eq!(longest_with_announcement(&str1, &str2, announcement), str2)
    }
}

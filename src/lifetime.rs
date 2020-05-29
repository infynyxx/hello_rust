// generic lifetimes

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

pub fn main() {
    print("omitted generics");
    print_expanded("expanded generics");
    trim("trim");
    trim_expanded("trim expanded");
    println!("{}", get_str());
}

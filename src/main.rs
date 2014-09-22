extern crate hello_rust;
use hello_rust::add_one;
use hello_rust::Circle;

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
    println!("add_one(100i) == {}", add_one(&100i));

    let c = Circle::new(0.0, 0.0, 2.0);
    println!("area == {}", c.area());
}

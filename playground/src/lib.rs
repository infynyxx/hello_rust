#[macro_use]
extern crate log;

extern crate assert_approx_eq;
extern crate tempfile;

pub mod closures;
pub mod datastructure;
pub mod enums;
pub mod error_handling;
pub mod games;
pub mod generics;
pub mod iter;
pub mod lifetime;
pub mod slice;
pub mod structs;

pub struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
}

fn add_three(x: i32) -> i32 {
    x + 3
}

fn times_four(x: i32) -> i32 {
    x * 4
}

pub fn add_one(x: &i32) -> i32 {
    *x + 1
}

pub fn add_three_times_four(x: i32) -> i32 {
    times_four(add_three(x))
}

pub struct Person {
    pub first: String,
    pub last: String,
}

pub fn hello() {
    let praj = Person {
        first: "Prajwal".to_string(),
        last: "Tuladhar".to_string(),
    };
    println!(
        "Size of person {} {} is {}",
        praj.first,
        praj.last,
        name_size(&praj)
    );
}

pub fn name_size(person: &Person) -> i32 {
    //let Person {first, last} = person;
    //first.len() + last.len()
    (person.first.len() + person.last.len()) as i32
}

#[cfg(test)]
mod test {
    use super::add_one;
    use super::add_three;
    use super::times_four;

    #[test]
    fn test_add_three() {
        let result = add_three(5i32);
        assert_eq!(8i32, result);
    }

    #[test]
    fn test_times_four() {
        let result = times_four(5i32);
        assert_eq!(20i32, result);
    }

    #[test]
    fn test_add_one() {
        let result = add_one(&100i32);
        assert_eq!(101i32, result);
    }
}

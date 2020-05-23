//use rand;
#[macro_use] extern crate log;

pub struct Circle {
    x: f64,
    y: f64,
    radius: f64
}

impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius
        }
    }
}

fn add_three(x: i32) -> i32 { x + 3 }

fn times_four(x: i32) -> i32 { x * 4 }

pub fn add_one(x: &i32) -> i32 { *x + 1 }

pub fn add_three_times_four(x: i32) -> i32 {
    times_four(add_three(x))
}


pub mod games {    
    use std::io;    

    pub fn random_guess() {
        println!("Guess the number!");
        loop {
            let secret_number = (rand::random::<u32>() % 100 as u32) + 1;
            println!("Enter your guess:");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).ok().expect("Failed to read line");

            let input_num = buffer.trim().parse::<u32>();
            debug!("input_num={:?} buffer={}", input_num, buffer);

            match input_num {
                Ok(num) => {
                    println!("You guessed {}", num);
                    println!("Secret number {}", secret_number);

                    match compare(num, secret_number) {
                        Ordering::LESS => println!("Too small!"),
                        Ordering::GREATER => println!("Too big"),
                        Ordering::EQUAL => {
                            println!("You win!");
                            return;
                        }
                    }
                },
                Err(_) => {
                    println!("Please input a number!");
                    continue;
                }
            };
        }
    }

    enum Ordering {
        LESS,
        GREATER,
        EQUAL
    }

    fn compare(a: u32, b: u32) -> Ordering {
        if a > b { Ordering::GREATER }
        else if a < b { Ordering::LESS }
        else { Ordering::EQUAL }
    }

}

pub struct Person {
    pub first: String,
    pub last: String
}

pub fn hello() {
    let praj = Person {
        first: "Prajwal".to_string(),
        last: "Tuladhar".to_string()
    };
    println!("Size of person {} {} is {}", praj.first,
        praj.last,
        name_size(&praj));
}

pub fn name_size(person: &Person) -> i32 {
    //let Person {first, last} = person;
    //first.len() + last.len()
    (person.first.len() + person.last.len()) as i32
}

pub struct Node {
    value: i32,
    next: Option<Box<Node>>
}

impl Node {
    pub fn print_value(&self) {
        println!("Value: {}", self.value);
        match self.next {
            Some(ref node) => {
                node.print_value();
            },
            None => {
                println!("Next node: None");
            }
        }
    }
}

pub fn recursive_data_structure() {
    let node1 = Node {value: 1, next: None};
    let node2 = Node {value: 2, next: Some(Box::new(node1))};
    let node3 = Node {value: 3, next: Some(Box::new(node2))};
    node3.print_value();
}


#[cfg(test)]
mod test {
    use super::add_three;
    use super::times_four;
    use super::add_one;

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

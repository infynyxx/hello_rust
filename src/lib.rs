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

fn add_three(x: int) -> int { x + 3 }

fn times_four(x: int) -> int { x * 4 }

pub fn add_one(x: &int) -> int { *x + 1 }

pub fn add_three_times_four(x: int) -> int {
    times_four(add_three(x))
}

pub mod games {
    use std::rand;
    use std::io;

    pub fn random_guess() {
        println!("Guess the number!");
        loop {
            let secret_number = (rand::random::<uint>() % 100u) + 1u;
            println!("Enter your guess:");
            let input = io::stdin().read_line()
                .ok()
                .expect("Failed to read line");

            let input_num: Option<uint> = from_str(input.as_slice().trim());

            let num = match input_num {
                Some(num)   => num,
                None        => {
                    println!("Please input a number!");
                    continue;
                }
            };

            println!("You guessed {}", input);
            println!("Secret number {}", secret_number);

            match compare(num, secret_number) {
                LESS => println!("Too small!"),
                GREATER => println!("Too big"),
                EQUAL => {
                    println!("You win!");
                    return;
                }
            }
        }
    }

    enum Ordering {
        LESS,
        GREATER,
        EQUAL
    }

    fn compare(a: uint, b: uint) -> Ordering {
        if a > b { GREATER }
        else if a < b { LESS }
        else { EQUAL }
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

pub fn name_size(person: &Person) -> uint {
    //let Person {first, last} = person;
    //first.len() + last.len()
    person.first.len() + person.last.len()
}

pub struct Node {
    value: uint,
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
    let node2 = Node {value: 2, next: Some(box node1)};
    let node3 = Node {value: 3, next: Some(box node2)};
    node3.print_value();
}


#[cfg(test)]
mod test {
    use super::add_three;
    use super::times_four;
    use super::add_one;

    #[test]
    fn test_add_three() {
        let result = add_three(5i);
        assert_eq!(8i, result);
    }

    #[test]
    fn test_times_four() {
        let result = times_four(5i);
        assert_eq!(20i, result);
    }

    #[test]
    fn test_add_one() {
        let result = add_one(&100i);
        assert_eq!(101i, result);
    }
}

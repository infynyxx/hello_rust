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

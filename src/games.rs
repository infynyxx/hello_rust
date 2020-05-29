pub mod games {
    use std::io;

    pub fn random_guess() {
        println!("Guess the number!");
        loop {
            let secret_number = (rand::random::<u32>() % 100 as u32) + 1;
            println!("Enter your guess:");
            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .ok()
                .expect("Failed to read line");

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
                }
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
        EQUAL,
    }

    fn compare(a: u32, b: u32) -> Ordering {
        if a > b {
            Ordering::GREATER
        } else if a < b {
            Ordering::LESS
        } else {
            Ordering::EQUAL
        }
    }
}

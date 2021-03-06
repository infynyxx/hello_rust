extern crate playground;

#[cfg(not(test))]
fn main() {
    use playground::add_one;
    use playground::hello;
    use playground::name_size;
    use playground::Circle;
    use playground::Person;

    use std::sync::mpsc;
    use std::thread;

    println!("Hello, world!");
    println!("add_one(100i) == {}", add_one(&100i32));

    let c = Circle::new(0.0, 0.0, 2.0);
    println!("area == {}", c.area());

    let nums = [1, 2];
    let noms = ["Jon", "John", "Ram", "Shyam"];
    let odds = nums.iter().map(|&x| x * 2 - 1);
    for num in odds {
        thread::spawn(move || {
            println!("{} says hello from lightweight thread", noms[num]);
        });
    }

    // closure
    let closure_args = |arg: i32| -> () {
        println!("closure_args: {}", arg + 1);
    };

    closure_args(10i32);
    closure_args(99i32);

    let closure = |x: i32, y: i32| -> () {
        println!("{} - {} - {}", c.area(), x, y);
    };

    closure(10, 11);

    let (channel, port) = mpsc::channel();
    thread::spawn(move || {
        let result = 5i32;
        channel.send(result);
    });

    let result = port.recv();
    println!("Result from task {}", result.unwrap());

    let mut count = 0i32;
    while count < 10i32 {
        count = count + 1i32;
        thread::spawn(move || {
            println!("Count {}", count);
        });
    }

    let msg = Some("Howdy");
    match msg {
        Some(ref m) => println!("{}", *m),
        None => (),
    }

    // procs
    /////////////
    let proc_variable = 25i32;
    let p = move || proc_variable * proc_variable;
    println!("proc value is {}", p());

    // vectors
    // /////
    let nums = vec![21i32, 22i32, 56i32];
    println!("nums: {:?}", nums);
    let mut nums_mutable = vec![1i32, 2i32, 3i32];
    nums_mutable.push(100i32);
    println!("mutable nums: {:?}", nums_mutable);
    for i in nums_mutable.iter() {
        println!("{}", i);
    }

    let slice = nums_mutable.as_slice();
    println!("slice: {:?}", slice);

    let person = Person {
        first: "Prajwal".to_string(),
        last: "Tuladhar".to_string(),
    };
    println!("length of person is {}", name_size(&person));
    hello();

    pub use crate::playground::datastructure::common;
    common::recursive_data_structure();

    pub use crate::playground::lifetime;
    lifetime::main();

    use crate::playground::games;
    games::games::random_guess();
}

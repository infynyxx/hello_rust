extern crate hello_rust;
use hello_rust::add_one;
use hello_rust::Circle;
use hello_rust::games;
use hello_rust::Person;
use hello_rust::name_size;
use hello_rust::hello;

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
    println!("add_one(100i) == {}", add_one(&100i));

    let c = Circle::new(0.0, 0.0, 2.0);
    println!("area == {}", c.area());


    let nums = [1, 2];
    let noms = ["Jon", "John", "Ram", "Shyam"];
    let mut odds = nums.iter().map(|&x| x * 2 - 1);
    for num in odds {
        spawn(proc() {
            println!("{} says hello from lightweight thread", noms[num]);
        });
    }

    // closure
    let closure_args = |arg: int| -> () {
        println!("closure_args: {}", arg + 1);
    };

    closure_args(10i);
    closure_args(99i);

    let closure = |x: int, y: int| -> () {
        println!("{} - {} - {}", c.area(), x, y);
    };

    closure(10, 11);

    let (channel, port) = channel();
    spawn(proc() {
        let result = 5i;
        channel.send(result);
    });

    let result = port.recv();
    println!("Result from task {}", result);

    let mut count = 0i;
    while count < 10i {
        count = count + 1i;
        spawn(proc() {
            println!("Count {}", count);
        });
    }

    let msg = Some("Howdy");
    println!("{}", msg);
    match msg {
        Some(ref m) => println!("{}", *m),
        None => ()
    }


    // procs
    /////////////
    let proc_variable = 25i;
    let p = proc() {
        proc_variable * proc_variable
    };
    println!("proc value is {}", p());

    // vectors
    // /////
    let nums = vec![21i, 22i, 56i];
    println!("nums: {}", nums);
    let mut nums_mutable = vec![1i, 2i, 3i];
    nums_mutable.push(100i);
    println!("mutable nums: {}", nums_mutable);
    for i in nums_mutable.iter() {
        println!("{}", i);
    }

    let slice = nums_mutable.as_slice();
    println!("slice: {}", slice);

    //games::random_guess();
    let person = Person{
        first: "Prajwal".to_string(),
        last: "Tuladhar".to_string()
    };
    println!("length of person is {}", name_size(&person));
    hello();
}

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = 5;
    loop {
        println!("Please input your guess.");
        if guess == 0 {
            println!(
                "You ran out of guessing turns. You lose. The secret number is {secret_number}"
            );
            break;
        }

        let mut input = String::new();

        // :: indicates an associated function of the specific thing.
        // For example: String::new() indicates that new() is the associated
        // function of String type.
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // We create a variable named "guess" even though there is already
        // a varibale with the with same name. The feature in here is
        // shadowing which really useful when convert from one type to another type

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if input.trim() == "quit" {
                    println!("Quitting the game. Bye!!!");
                    break;
                } else {
                    println!("Please type number!!!");
                    continue;
                }
            }
        };

        println!("You guess: {input}");

        // match expression contains arms. There are 3 arms below.
        guess -= 1;
        match input.cmp(&secret_number) {
            // The whole statement A => B is called arm.
            // where A is pattern and B is expression.
            Ordering::Less => println!("Guess Larger!, {guess} guess(es) left"),
            Ordering::Equal => {
                println!("You are correct, tada!!!");
                break;
            }

            Ordering::Greater => println!("Guess Smaller!, {guess} guess(es) left"),
        }
    }
}

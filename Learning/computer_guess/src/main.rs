use rand::Rng;
use std::io;

fn clear_screen() {
    // ANSI escape code for clearing the screen
    print!("\x1B[2J\x1B[1;1H");
    // The first part "\x1B[2J" clears the entire screen
    // The second part "\x1B[1;1H" positions the cursor at the top-left corner
}

fn main() {
    println!("Welcome to the Guess My Number game!");
    println!("Think of a number between 0 and 99 and I will guess in 7 attempts");

    let mut min = 0;
    let mut max = 100;
    let mut count = 7;
    let mut guess = 0;

    'outer: loop {
        let prev_guess = guess;
        guess = match count {
            0 => {
                println!("Sorry, I couldn't guess your number in 5 attempts.");
                break;
            }
            1 => rand::thread_rng().gen_range(min..=max),
            _ => (min + max) / 2,
        };

        println!("I guess {guess} is your number!!!");
        loop {
            println!("Is that your number? ((H)igher/(L)ower/(C)orrect)");

            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");

            let answer = answer.trim();

            if (guess == prev_guess) && (answer != String::from("C") || answer != String::from("c"))
            {
                println!("You are lying. I correctly guessed your number but you don't admit it.");
                break 'outer;
            }

            count -= 1;
            match answer.to_lowercase().trim() {
                "h" => {
                    min = guess + 1;
                    clear_screen();
                    break;
                }
                "l" => {
                    max = guess - 1;
                    clear_screen();
                    break;
                }
                "c" => {
                    println!("I'm really good at guessing game!!!");
                    break 'outer;
                }
                _ => {
                    continue;
                }
            }
        }
    }
}

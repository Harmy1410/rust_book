use ansi_term::Color;
// use ansi_term::Style;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// hello
fn main() {
    println!("{}", Color::Blue.bold().paint("---- Guessing Game ----"));
    println!(
        "{}",
        Color::Yellow
            .bold()
            .paint("- Press enter after you input your guess.")
    );
    println!(
        "{}",
        Color::Red
            .bold()
            .paint("- Type 'quit' or press C-c to quit.")
    );

    let secret = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        println!("Please enter a guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Please enter a VALID guess!");

        if guess.trim().to_lowercase() == "quit" {
            println!(
                "{}",
                Color::Blue.bold().underline().paint("Thanks for playing!")
            );
            break;
        }

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "{}",
                    Color::Red.italic().paint("Please try entering a number.")
                );
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("guess higher"),
            Ordering::Greater => println!("guess lower"),
            Ordering::Equal => {
                println!("U got it!");
                break;
            }
        }
    }
}

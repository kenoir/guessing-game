extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("|-----------------|");
    println!("|Guess the number!|");
    println!("|-----------------|");
    println!("");
    println!("");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("DO NOT BELIEVE THE SECRET IS {}", secret_number);

    loop {
        println!("FEED MY YOUR GUESSES HUMAN");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("WHERE IS MY GUESS MEATBAG");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("HA HA HA FLESHY FOOL THAT IS NOT A NUMBER");
                continue;
            }
        };

        println!("YOU HAVE FOOLISHLY GUESSED: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("YOUR NUMBERS ARE SMALL LIKE YOUR PUNY MIND"),
            Ordering::Greater => println!("YOUR NUMBERS ARE FOOLISHLY LARGE LIKE YOUR MEATY HEAD"),
            Ordering::Equal => {
                println!("!*!*!*!*!*!*!**!*!*!*!NUMBER EQUALITY ERROR --- EMERGENCY SHUTDOWN");
                break;
            }
        }
    }
}

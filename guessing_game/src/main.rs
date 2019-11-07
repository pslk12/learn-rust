extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let rng = rand::thread_rng().gen_range(1, 101);
    println!("Secret Number is {}", rng);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", &guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&rng) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

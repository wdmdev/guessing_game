use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count = 0;

    loop {
        println!("Please input your guess.");
        count += 1;

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        //Shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was too low!"),
            Ordering::Greater => println!("Your guess was too high!"),
            Ordering::Equal => {
                println!("Your guess was spot on, you win!");
                println!("You used {count} attempts!");
                break;
            }
        }
    }

}

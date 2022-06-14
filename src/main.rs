use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut attempt_number = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input a number!");
                    continue;
                }
            };

        println!("You guessed: {}", guess);

        attempt_number += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                if (guess + 9) < secret_number {
                    println!("Too Too small!");
                } else {
                    println!("Too small!");
                }
            }
            Ordering::Greater => {
                if (guess - 9) > secret_number {
                    println!("Too Too big!");
                } else {
                    println!("Too big!");
                }
            }
            Ordering::Equal => {
                if attempt_number == 1 {
                    println!("You win but stop cheating dude plz!");
                } else {
                    println!("You win in {} attempt!", attempt_number);
                }
                break;
            }
        }
    }
}

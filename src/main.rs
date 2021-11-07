use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Welcome in the Guess Game");
    println!("Try to guess my number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please enter a guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win! The result was {}", secret_number);
                break;
            },
        }

        println!("You guessed: {}", guess);
    }
}
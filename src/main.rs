use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Welcome in the Guess Game");
    println!("Try to guess my number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut attempts: Vec<u32> = Vec::new();

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
            Ordering::Less => {
                println!("Too small");
                attempts.insert(attempts.len(), guess);
            },
            Ordering::Greater => {
                println!("Too big");
                attempts.insert(attempts.len(), guess);
            },
            Ordering::Equal => {
                println!("You win! The result was {}", secret_number);
                println!("Total attempts: {} {:?}", attempts.len(), attempts);
                break;
            },
        }

        println!("You guessed: {}", guess);
    }
}
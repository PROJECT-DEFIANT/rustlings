use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guessing game starts now !!!");
    println!("Pleae input your guess...");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!, try again"),
            Ordering::Greater => println!("Way too much, try again"),
            Ordering::Equal => {
                println!("Congrats ! You have found it");
                break;
            } 
        }
    }

}

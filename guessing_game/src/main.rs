use std::io;

use std::cmp::Ordering;
use rand;
use rand::Rng;

fn main()
{
    let apples = 5; // immutable
    let mut bananas = 5; // mutable
    bananas += 5;
    println!("Apples = {apples} and Bananas = {}", bananas + 2);

    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);


    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=10);

    //println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}

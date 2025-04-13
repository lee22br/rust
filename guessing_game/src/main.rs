use std::io;
fn main()
{
    //let apples = 5; // immutable
    //let mut bananas = 5; // mutable

    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");



    println!("You guessed: {}", guess);
}

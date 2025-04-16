use ferris_says::say;
use std::io::{stdout, BufWriter};
fn main()
{
    println!("FIRST RUST APP RUNNING....");
    //declare var String
    let mut hello_world = String::from("Hello World!");

    //concat char to end of String
    hello_world.push('!');

    // Print text to the console.
    println!("{}", hello_world);
    hello_world.push_str(" of Universe");
    println!("{}", hello_world);

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

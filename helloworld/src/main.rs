
fn main()
{
    //declare var String
    let mut hello_world = String::from("Hello World!");

    //concat char to end of String
    hello_world.push('!');

    // Print text to the console.
    println!("{}", hello_world);
    hello_world.push_str(" of Universe");
    println!("{}", hello_world);
}

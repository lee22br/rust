fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Time in seconds: {}",THREE_HOURS_IN_SECONDS);

    //MAX int numbers
    let number8b: u8 = 255;
    let number16b: u16 = 65535;
    let number32b: u32 = 2147483647;
    let number64b: u64 =  18446744073709551615;
    let num_arch: usize = 18446744073709551615;

    println!("Number: {}",num_arch);

    //Float number
    let float32: f32 = 3.14;
    let float64 = 1.1;
    println!("Float32: {}",float32);

    //Math operations
    // addition
    let sum = 5 + 10;
    println!("sum: {}",sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {}",difference);

    // multiplication
    let product = 4 * 30;
    println!("product: {}",product);

    // division
    let quotient = 56.7 / 32.2;
    println!("division float: {}",quotient);
    let truncated = -5 / 3; // Results in -1
    println!("division int: {}",truncated);

    // remainder (Modulo)
    let remainder = 43 % 5;
    println!("remainder int: {}",remainder);

}

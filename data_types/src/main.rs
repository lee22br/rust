fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Time in seconds: {}", THREE_HOURS_IN_SECONDS);

    //MAX int numbers
    let _number8b: u8 = 255;
    let _number16b: u16 = 65535;
    let _number32b: u32 = 2147483647;
    let _number64b: u64 = 18446744073709551615;
    let num_arch: usize = 18446744073709551615;
    println!("Number: {}", num_arch);

    //Float number
    let _float32: f32 = 3.14;
    let float64 = 1.1;
    println!("Float32: {}", float64);

    //Math operations
    // addition
    let sum = 5 + 10;
    println!("sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("division float: {}", quotient);
    let truncated = -7 / 3; // Results in -1
    println!("division int: {}", truncated);

    // remainder (Modulo)
    let remainder = 14 % 5;
    println!("remainder int: {}", remainder);

    let boolean = true;
    let _c = 'c';
    let cat = '😻';
    if boolean {
        println!("cat : {}", cat);
    }

    //Tuple
    let name_age: (&str, u16) = ("Leandro", 42);
    println!("Name:{} age: {}", name_age.0, name_age.1);

    //Arrays
    let array: [i16; 4] = [22, 35, 47, 15];
    for element in array {
        println!("Arrays: {element}");
    }
    //functions
    print_array(&array);
    println!("fn +1: {}",plus_one(10));
    println!("fn +2: {}",plus_two(10));
    println!("fn +3: {}",plus_three(10));

    //Control Flow
    if product < 100 {
        println!("product : {}", product);
    } else if product == 120{
        println!("product : {}", product);
    } else {
        println!("product over 100");
    }

    match remainder {
        0 => println!("Number divisible by 5"),
        1 | 2 | 3  => println!("Remainder 1..4"),
        _ => println!("Remainder : {}", remainder),
    }

}

//Functions
fn print_array(array: &[i16]) {
    for i in 0..array.len() {
        println!("Arrays by index: {}", array[i]);
    }
}
fn plus_one(x: i32) -> i32 {
    return x + 1;
}
fn plus_two(x: i32) -> i32 {
    x + 2
}
fn plus_three(x: i32) -> i32 {
     plus_one(x) + plus_two(x)
}
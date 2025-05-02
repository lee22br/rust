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

    //Like Switch
    match remainder {
        0 => println!("Number divisible by 5"),
        1 | 2 | 3  => println!("Remainder 1..3"),
        _ => println!("Remainder : {}", remainder),
    }

    //Loop
    let mut count = 0;
    loop {
        println!("Count: {}", count);
        if count == 2{
            break;
        }
        count += 1;
    }

    //loop
    count = 0;
    let result = loop {
        count += 1;

        if count == 5 {
            break count * 2;
        }
    };
    println!("The result is {}",result);

    //loop with label
    count = 0;
    'loop_out:loop{
        println!("Count: {}", count);
        let mut remaining = 5;
        loop {
            println!("Count: {}", count);
            if remaining == 4 {
                break;
            }
            if count == 2 {
                break 'loop_out;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("Count is {}",count);

    //while
    count = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }

    //loop in collections
    let vet = [10, 21, 4, 22,333];
    for element in vet {
        println!("Arrays: {}", element);
    }

    //Variable Scope
    let s = String::from("hello");
    {
        let s = "hello2";
        println!("String: {}", s);
    }
    println!("String: {}", s);

    let s1 = String::from("hello");
    let s2 = s1; //its like move s1 to s2, s1 now is out of scope
    println!("s2: {}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}", s1);
    println!("s2: {}", s2);

    // using int its work
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    //Ownership and Functions
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");

    //reference
    let mut hello_world = String::from("hello");
    change(&mut hello_world);
    println!("Change by Ref to {}", hello_world);
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

// function by value (ownership)
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

//function by reference
fn change(text: &mut String) {
    text.push_str(", world");
}
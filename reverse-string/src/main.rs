fn main() {
    let input = "LeandroIglezias";
    println!("Call Revers function: {}",input);
    println!("Result:  {}",reverse(&input));
}

pub fn reverse(input: &str) -> String {
    return input.chars().rev().collect::<String>();
}
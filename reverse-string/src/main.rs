fn main() {
    println!("Hello, world!");
    reverse("TESTE");
}

pub fn reverse(input: &str) -> String {
    let result: [char] = input.chars().rev().collect();
    return result.iter().collect::<String>();
}
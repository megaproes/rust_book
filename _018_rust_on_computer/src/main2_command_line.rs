use std::env::args;
enum Letters {
    Capitalize,
    Lowercase,
    Nothing,
}
fn main() {
    let mut changes = Letters::Nothing;
    let input = args().collect::<Vec<_>>();
    if let Some(arg) = input.get(1) {
        match arg.as_str() {
            "capital" => changes = Letters::Capitalize,
            "lowercase" => changes = Letters::Lowercase,
            _ => {}
        }
    }
    for word in input.iter().skip(2) {
        match changes {
            Letters::Capitalize => println!("{}", word.to_uppercase()),
            Letters::Lowercase => println!("{}", word.to_lowercase()),
            _ => println!("{}", word),
        }
    }
}

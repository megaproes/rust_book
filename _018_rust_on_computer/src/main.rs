use std::{env::args, io};
fn main() {
    println!("Please type something, or x to escape:");
    let mut input_string = String::new();
    while input_string.trim() != "x" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        println!("You wrote {input_string}");
    }
    println!("See you later!");

    println!("{:?}", std::env::args());
    let input = args();
    for entry in input {
        println!("You entered: {}", entry);
    }
    
    let input = args();
    input.skip(1).for_each(|item| {
        println!(
            "You wrote {item}, which in capital letters is {}",
            item.to_uppercase()
        );
    });
}

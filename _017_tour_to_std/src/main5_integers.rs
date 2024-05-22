use rand::random;

fn add_numbers(one: u8, two: u8) {
    match one.checked_add(two) {
        Some(num) => println!("Added {one} to {two}: {num}"),
        None => println!("Error: couldn't add {one} to {two}"),
    }
}

fn main() {
    // let some_number = 200_u8;
    // println!("{}", some_number + 200);

    for _ in 0..3 {
        let some_number = random::<u8>();
        let other_number = random::<u8>();
        add_numbers(some_number, other_number);
    }
}

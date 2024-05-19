use std::time::Instant;
fn main() {
    let start_of_main = Instant::now();
    let before_operation = Instant::now();
    let mut new_string = String::new();
    loop {
        new_string.push('წ');
        if new_string.len() > 100_000 {
            break;
        }
    }
    let after_operation = Instant::now();
    println!("{:?}", before_operation - start_of_main);
    println!("{:?}", after_operation - start_of_main);
    // or:
    let start = Instant::now();
    println!("Time elapsed before busy operation: {:?}", start.elapsed());
    let mut new_string = String::new();
    loop {
        new_string.push('წ');
        if new_string.len() > 100_000 {
            break;
        }
    }
    println!("Operation complete. Time elapsed: {:?}", start.elapsed());
}

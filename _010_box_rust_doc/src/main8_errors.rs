fn handle_error_inside_function() {
    println!("{:?}", "seven".parse::<i32>());
}

use std::error::Error;
fn parse_numbers(int: &str, float: &str) -> Result<f64, Box<dyn Error>> {
    let num_1 = int.parse::<i32>()?;
    let num_2 = float.parse::<f64>()?;
    Ok(num_1 as f64 + num_2)
}
fn main() {
    handle_error_inside_function();

    let my_number = parse_numbers("8", "ninepointnine").unwrap();
}

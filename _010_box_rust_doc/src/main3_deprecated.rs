#[deprecated(note = "Always panics for some reason, not sure why. Please use new_function instead")]
fn old_function() {
    panic!();
    println!("Works well");
}
fn new_function() {
    println!("Works well");
}
fn main() {
    old_function();
}

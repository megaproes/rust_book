use std::mem;
fn main() {
    println!("Size of an i32: {}", mem::size_of::<i32>());
    let my_array = [8; 50];
    println!("Size of this array: {}", mem::size_of_val(&my_array));
    let some_string = String::from("Droppable because it's not Copy");
    drop(some_string);
    // some_string.clear();
}

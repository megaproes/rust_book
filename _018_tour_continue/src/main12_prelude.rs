#![no_implicit_prelude]
fn main() {
//     let my_vec = vec![8, 9, 10];
//     let my_string = String::from("This won't work");
//     println!("{my_vec:?}, {my_string}");

    extern crate std;
    use std::convert::From;
    let my_vec = std::vec![8, 9, 10];
    let my_string = std::string::String::from("This won't work");
    std::println!("{my_vec:?}, {my_string}");
}

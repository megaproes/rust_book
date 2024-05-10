fn prints_str(my_str: &str) {
    println!("{my_str}");
}
fn main() {
    let my_string = String::from("I am a string");
    prints_str(&my_string);
}
fn works() -> &'static str {
    "I live forever!"
}
// fn does_not_work() -> &'static str {
// &String::from("Sorry, I only live inside the fn. Not 'static")
// }

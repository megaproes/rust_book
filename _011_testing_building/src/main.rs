mod print_things {
    use std::fmt::Display;
    fn prints_one_thing<T: Display>(input: T) {// can't use unless it public {
        println!("{input}");
    }
}
fn main() {}

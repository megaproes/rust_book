#![allow(dead_code)]
#![allow(unused_variables)]
struct Struct1 {}
struct Struct2 {}
struct Struct3 {}
struct Struct4 {}
struct Struct5 {}
fn main() {
    let char1 = 'ん';
    let char2 = ';';
    let some_str = "I'm just a regular &str";
    let some_vec = vec!["I", "am", "just", "a", "vec"];

    let my_string = HoldsAString {
        the_string: "Here I am!".to_string(),
    };
    println!("{:?}", my_string);
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Clone)]
struct HoldsAString {
    the_string: String,
}

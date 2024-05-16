#[derive(Clone, Copy)]
struct NumberAndBool {
    number: i32,
    true_or_false: bool,
}
fn does_nothing(input: NumberAndBool) {}
fn main() {
    let number_and_bool = NumberAndBool {
        number: 8,
        true_or_false: true,
    };
    does_nothing(number_and_bool);
    does_nothing(number_and_bool);
}

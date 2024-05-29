//  Use * to signify any number of repetitions.
//  Use + to signify any number of repetitions (but at least one).
//  Use ? to signify zero or one occurrence.

macro_rules! comma_check {
    () => {
        println!("Got nothing!");
    };
    ($input:expr) => {
        println!("One expression!")
    };
    ($input:expr $(,)?) => {
        println!("One expression with a comma at the end!")
    };
    ($input:expr $(,)? $(,)?) => {
        println!("One expression with two commas at the end!")
    };
    ($input:expr $(;)? $(,)?) => {
        println!("One expression with a semicolon and a comma!")
    };
}
fn main() {
    comma_check!();
    comma_check!(8);
    comma_check!(8,);
    comma_check!(8,,);
    comma_check!(8;,);
}

use std::cmp::PartialOrd;
use std::fmt::Display;
fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, input_1: U, input_2: U) {
    println!(
        "{statement}! Is {input_1} greater than {input_2}? {}",
        input_1 > input_2
    );
}

// Easier to read with 'where'
fn compare_and_display2<T, U>(statement: T, input_1: U, input_2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
    println!(
        "{statement}! Is {input_1} is greater than {input_2}? -- {}",
        input_1 > input_2
    );
}
fn main() {
    compare_and_display("Listen up!", 9, 8);
    compare_and_display2("Listen up!", 9, 8);
	
	// U and T can be different types or may be the same types.
    say_two("Hello there!", String::from("I hate sand."));
    say_two(String::from("Where is Padme?"), String::from("Is she all right?"));
}

fn say_two<T: Display, U: Display>(statement_1: T, statement_2: U) {
    println!("I have two things to say: {statement_1} and {statement_2}");
}

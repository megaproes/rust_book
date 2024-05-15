use std::fmt::Display;

fn print_maximum(one: i32, two: i32) {
    let higher = if one > two { one } else { two };
    println!("{higher} is higher");
}

fn print_maximum2<T: PartialOrd + Display>(one: T, two: T) {
    let higher = if one > two { one } else { two };
    println!("{higher} is higher.");
}

fn prints_it(input: impl Into<String> + Display) {
    println!("You can print many things, including {input}");
}
fn main() {
    print_maximum(8, 10);
    print_maximum2(8, 10);

    let name = "Tuon";
    let string_name = String::from("Tuon");
    prints_it(name);
    prints_it(string_name);
}
fn apply_fn(x: i32) -> impl Fn(i32) -> i32 {
     |y| x + y // need move here since 'x' isn't safe and the closure may outlife the function ('x' is stack alocated)
}
// fn apply_fn2<F>(x: i32) -> F
// where
//     F: Fn(i32) -> i32,
// {
//     move |y| x + y
// }

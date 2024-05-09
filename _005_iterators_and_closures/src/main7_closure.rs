fn main() {
    let my_closure = || println!("This is a closure");
    my_closure();

    let my_closure = |x: i32| println!("{x}");
    my_closure(5);
    my_closure(5 + 5);

    let my_closure = || {
        let number = 7;
        let other_number = 10;
        println!("The two numbers are {number} and {other_number}.");
    };
    my_closure();

    let number_one = 6;
    let number_two = 10;
    let my_closure = || println!("{}", number_one + number_two);
    my_closure();

    let number_one = 6;
    let number_two = 10;
    let my_closure = |x: i32| println!("{}", number_one + number_two + x);
    my_closure(5);
}

use std::borrow::Cow;

fn greeting(name: &str) -> Cow<'_, str> {
    if name.starts_with('A') {
        // If the name starts with 'A', no modification needed, borrow it
        Cow::Borrowed(name)
    } else {
        // Otherwise, create a new string with a prefix and own it
        Cow::Owned(format!("Hello, {}", name))
    }
}

fn main() {
    let name_a = "Alice";
    let name_b = "Bob";

    let greeting_a: Cow<str> = greeting(name_a);
    let greeting_b: Cow<str> = greeting(name_b);

    println!("Greeting A: {}", greeting_a);
    println!("Greeting B: {}", greeting_b);

    let even_numbers = vec![2, 4, 6];
    let odd_numbers = vec![1, 3, 5];

    let adjusted_even = adjust_numbers(&even_numbers);
    let adjusted_odd = adjust_numbers(&odd_numbers);

    println!("Adjusted Even: {:?}", adjusted_even);
    println!("Adjusted Odd: {:?}", adjusted_odd);
    
    let somestr = "sdddda";
    let mut another_str = somestr.to_owned();
    another_str = "sadinstead".to_string();
    println!("{}", somestr);
}

fn adjust_numbers(numbers: &[i32]) -> Cow<'_, [i32]> {
    if numbers.iter().all(|&x| x % 2 == 0) {
        // If all numbers are even, just borrow them
        Cow::Borrowed(numbers)
    } else {
        // If any number is odd, increase all numbers by 1 and own the result
        let adjusted: Vec<i32> = numbers.iter().map(|x| x + 1).collect();
        Cow::Owned(adjusted)
    }
}

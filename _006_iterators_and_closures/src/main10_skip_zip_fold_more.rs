fn main() {
    let even_odd_iter = ["even", "odd"].into_iter().cycle();
    let even_odd_vec: Vec<(i32, &str)> = (0..=5).zip(even_odd_iter).collect();
    println!("{:?}", even_odd_vec);
    assert_eq!(even_odd_vec[0], (0, "even"));
    assert_eq!(even_odd_vec[1], (1, "odd"));
    assert_eq!(even_odd_vec[2], (2, "even"));
    assert_eq!(even_odd_vec[3], (3, "odd"));
    assert_eq!(even_odd_vec[4], (4, "even"));
    assert_eq!(even_odd_vec[5], (5, "odd"));

    let ten_chars: Vec<char> = ('a'..).take(10).collect();
    let skip_then_ten_chars: Vec<char> = ('a'..).skip(1300).take(10).collect();
    println!("{ten_chars:?}");
    println!("{skip_then_ten_chars:?}");

    let some_numbers = vec![9, 6, 9, 10, 11];
    println!(
        "{}",
        some_numbers
            .iter()
            .fold(0, |total_so_far, next_number| total_so_far + next_number)
    );
    
    
}

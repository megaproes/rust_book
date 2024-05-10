fn main() {
    let num_array = ["8", "9", "Hi", "9898989898"];
    let mut char_vec = vec![];
    for index in 0..5 {
        char_vec.push(
            num_array
                .get(index)
                .and_then(|number| number.parse::<u32>().ok())
                .and_then(|number| char::try_from(number).ok()),
        );
    }
    // char_vec.retain(|&x| x.is_some());

    println!("{:?}", char_vec);
    
    fn sq_then_to_string(x: u32) -> Option<String> {
        x.checked_mul(x).map(|sq| sq.to_string())
    }
    
    assert_eq!(Some(2).and_then(sq_then_to_string), Some(4.to_string()));
    assert_eq!(Some(1_000_000).and_then(sq_then_to_string), None); // overflowed!
    assert_eq!(None.and_then(sq_then_to_string), None);
}

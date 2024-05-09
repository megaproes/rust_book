use std::collections::HashMap;
fn main() {
    let some_keys = vec![0, 1, 2, 3, 4, 5];
    let some_values = vec!["zero", "one", "two", "three", "four", "five"];
    let number_word_hashmap: HashMap<i32, &str> = some_keys
        .into_iter()
        .zip(some_values.into_iter())
        .collect::<HashMap<_, _>>();
    println!(
        "The value at key 2 is: {}",
        number_word_hashmap.get(&2).unwrap()
    );
    // or:
    let some_numbers = vec![0, 1, 2, 3, 4, 5];
    let some_words = vec!["zero", "one", "two", "three", "four", "five"];
    let number_word_hashmap: HashMap<_, _> = some_numbers
        .into_iter()
        .zip(some_words.into_iter())
        .collect(); // so we don;t put the type here

    // or:
    let keys = vec![0, 1, 2, 3, 4, 5].into_iter();
    let values = vec!["zero", "one", "two", "three", "four", "five"].into_iter();
    let number_word_hashmap: HashMap<i32, &str> = keys.zip(values).collect();
    println!(
        "The value at key 2 is: {}",
        number_word_hashmap.get(&2).unwrap()
    );

    
    
    // If both iterators have roughly equivalent syntax, it may be more readable to use [zip]:
    let a = [1, 2, 3];
    let b = [2, 3, 4];

    let mut zipped = std::iter::zip(
        a.into_iter().map(|x| x * 2).skip(1),
        b.into_iter().map(|x| x * 2).skip(1),
    );

    assert_eq!(zipped.next(), Some((4, 6)));
    assert_eq!(zipped.next(), Some((6, 8)));
    assert_eq!(zipped.next(), None);

    // instead of
    let mut zipped = a
        .into_iter()
        .map(|x| x * 2)
        .skip(1)
        .zip(b.into_iter().map(|x| x * 2).skip(1));
}

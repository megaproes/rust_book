fn main() {
    (1..=3).for_each(|num| println!("{num}"));

    (1..=3).for_each(|num| {
        println!("Got a {num}!");
        if num % 2 == 0 {
            println!("It's even")
        } else {
            println!("It's odd")
        };
    });

    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or_else(|| {
        if let Some(val) = my_vec.get(2) {
            val
        } else {
            &0
        }
    });
    println!("{fourth}");

    let char_vec = vec!['z', 'y', 'x'];
    char_vec
        .iter()
        .enumerate()
        .for_each(|(index, c)| println!("Index {index} is: {c}"));
}

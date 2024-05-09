fn main() {
    let numbers_together = "140399923481800622623218009598281";
    for (index, num) in numbers_together.char_indices() {
        match (index % 3, num) {
            (0 | 1, num) => print!("{num}"),
            _ => print!("{num}\t"),
        }
    }

    let my_vec = vec![8, 9, 10];
    my_vec
        .iter()
        .for_each(|_| println!("We didn't use the variables at all"));
}

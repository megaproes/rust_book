fn main() {
    let mut push_string = String::new();
    for _ in 0..100_000 {
        let capacity_before = push_string.capacity();
        push_string.push_str("I'm getting pushed into the string!");
        let capacity_after = push_string.capacity();
        if capacity_before != capacity_after {
            println!("Capacity raised to {capacity_after}");
        }
    }
    // We had to reallocate (copy everything over) 18 times. But now we know the final
    // capacity. So we’ll give it the capacity right away, and we don’t need to reallocate—just
    // one String capacity is enough:
    let mut push_string = String::with_capacity(4587520);
    for _ in 0..100_000 {
        let capacity_before = push_string.capacity();
        push_string.push_str("I'm getting pushed into the string!");
        let capacity_after = push_string.capacity();
        if capacity_before != capacity_after {
            println!("Capacity raised to {capacity_after}");
        } // We know the exact number in this case. Even if you only
          // have a general idea (like “at least 10,000”), you could
          // still use with_capacity() to avoid too many allocations.
    }

    // Of course, the actual length is certainly smaller than the final 4,587,520, which is
    // simply a doubling of the previous capacity when it was 2,293,760. We can shrink it,
    // though, with .shrink_to_fit(), which is another Vec method. But only do this once
    // you are sure of the final length because the capacity will double again even if you push
    // a single extra char to the Vec:
    let mut push_string = String::with_capacity(4587520);
    for _ in 0..100_000 {
        push_string.push_str("I'm getting pushed into the string!");
    }
    println!(
        "\n\nCurrent capacity as expected: {}",
        push_string.capacity()
    );
    push_string.shrink_to_fit();
    println!("Actual needed capacity: {}", push_string.capacity());
    push_string.push('a');
    println!("Whoops, it doubled again: {}", push_string.capacity());
    push_string.shrink_to_fit();
    println!(
        "Shrunk back to actual needed capacity: {}",
        push_string.capacity()
    );

    let mut my_string = String::from(".daer ot drah tib elttil a si gnirtssihT");
    while let Some(c) = my_string.pop() {
        print!("{c}");
    }

    // One convenient method for String is .retain(), which is a little bit like the
    // .filter() method we know for iterators. This method passes in a closure that we can
    // use to evaluate whether to keep each character or not. The following code keeps only
    // the characters inside a String that are letters or spaces:
    println!("\n");
    let mut my_string = String::from("Age: 20 Height: 194 Weight: 80");
    my_string.retain(|ch| ch.is_alphabetic() || ch == ' ');
    dbg!(my_string);
}

// pub fn pop(&mut self) -> Option<char> {
//     let ch = self.chars().rev().next()?;
//     let newlen = self.len() - ch.len_utf8();
//     unsafe {
//         self.vec.set_len(newlen);
//     }
//     Some(ch)
// }

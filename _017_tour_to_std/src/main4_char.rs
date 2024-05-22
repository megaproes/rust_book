use rand::random;

fn main() {
    let korean_word = "청춘예찬";
    for character in korean_word.chars() {
        print!("{} ", character.escape_unicode());
    }

    println!("This will always work: {}", char::from(100));
    println!("So will this: {}", char::from(random::<u8>()));
    for _ in 0..100_000 {
        if let Ok(successful_character) = char::try_from(random::<u32>()) {
            print!("{successful_character}");
        }
    }
    println!("\n\n");
    "Hi, привіт, 안녕, "
        .chars()
        .for_each(|c| println!("{c}: {}", c.len_utf8()));
}

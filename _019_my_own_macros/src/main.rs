macro_rules! pure_nonsense {
    (Hi Calvin.) => {
    GRITTINGS. MA NAM IS KAHLFIN. HEERYOR LUNBOKS. HOFFA GUT TAY ASKOOL.
    };
}

macro_rules! give_six {
    () => {
        6
    };
}
macro_rules! six_or_print {
    (6) => {
        6
    };
    () => {
        println!("You didn't give me 6.");
    };
}
macro_rules! might_print {
    (THis is strange input 하하はは哈哈 but it still works) => {
        println!("You guessed the secret message!")
    };
    () => {
        println!("You didn't guess it");
    };
}
fn main() {
    let six = give_six!();
    println!("{}", six);

    // let my_number = 10;
    // match my_number {
    //     10 => println!("You got a ten"),
    //     _ => 10,
    // } // this won't work!!! It will complain that you want to return () in one case and an i32 in the other
    // but a macro has nothing to do with code compilation, so it is fine
    // with producing a completely different output from a different match arm.
    let my_number = six_or_print!(6);
    six_or_print!();

    // six_or_rint!(66) // error! there is no _ wildcard.

    // This is another interesting point: the 6 this macro can take as input isn’t even an
    // i32; it’s just the number 6—a token. A token doesn’t have to be just ascii or numbers either

    might_print!(THis is strange input 하하はは哈哈 but it still works);
    might_print!();
}

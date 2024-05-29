// The unreachable! macro is kind of like todo! except it’s for code that will never be
// executed. Maybe you have a match in an enum that you know will never choose one of
// the arms, so the code can never be reached. If that’s so, you can write unreachable!
// so the compiler knows that it can ignore that part.

enum Banks {
    BankOfAmerica,
    Hsbc,
    Citigroup,
    DeutscheBank,
    TorontoDominionBank,
    SiliconValleyBank, // And so on...
}
fn get_swift_code(bank: &Banks) -> &'static str {
    use Banks::*;
    match bank {
        BankOfAmerica => "BOFAUS3N",
        Hsbc => "HSBCHKHHXXX",
        Citigroup => "CITIUS33XXX",
        DeutscheBank => "DEUTINBBPBC",
        TorontoDominionBank => "TDOMCATTTOR",
        SiliconValleyBank => unreachable!(),
    }
}

use rand::{thread_rng, Rng};
fn zero_to_three() -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0..=3)
}
fn human_readable_rand_num() -> &'static str {
    match zero_to_three() {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => unreachable!(),
    }
}

pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}
pub fn handle_season(season: Season) {
    use Season::*;
    match season {
        Spring => println!("Spring"),
        summer => println!("Summer"), // typo here and below
        Autumn => println!("Autumn"),
        Winter => println!("Winter"),
    }
}

enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}
// #[derive(Debug)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}
fn match_mood(mood: &Mood) -> i32 {
    // use Mood::*; -- to write without Mood::
    let happiness_level = match mood {
        Mood::Happy => 10,
        Mood::Sleepy => 6,
        Mood::NotBad => 7,
        Mood::Angry => 2,
    };
    happiness_level
}
fn main() {
    let my_mood = Mood::NotBad;
    let happiness_level = match_mood(&Mood::NotBad);
    println!("Out of 1 to 10, my happiness is {happiness_level}");

    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("{:?}", season as u32);
    }

    use Star::*;
    let starvec: Vec<Star> = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];
    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star."),
            size if size >= 80 && size <= 200 => println!("This is a good-sized star."),
            other_size => println!("That star is pretty big! It's {other_size}"),
        }
    }
}

enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

struct ColorRgb(u8, u8, u8);
enum Climate {
    Tropical,
    Dry,
    Temperature,
    Continental,
    Polar,
}
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
    climate: Climate,
}
enum ThingsInTheSky {
    Sun,
    Stars,
}
fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars,
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun!"),
        ThingsInTheSky::Stars => println!("I can see the stars!"),
    }
}
fn main() {
    let my_color = ColorRgb(50, 0, 50);
    println!("The second part of the color is: {}", my_color.1);

    let kalmykia = Country {
        population: 500_000,
        capital: "Elista".to_string(),
        leader_name: "Batu Khasikov".to_string(),
        climate: Climate::Continental,
    };
    
    let time =  8;
    let sky_state = create_skystate(time);
    check_skystate(&sky_state)
}

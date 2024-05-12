fn takes_a_string(_unused_string: String) {}
#[derive(Debug)]
struct City {
    name: String,
    population: u32,
    city_history: String,
}
#[derive(Debug)]
struct CityData {
    names: Vec<String>,
    histories: Vec<String>,
}
fn main() {
    //     let user_name = String::from("User MacUserson");
    //     takes_a_string(user_name);
    //     takes_a_string(user_name); // we can't use because the above lines moves user_name

    let calgary = City {
        name: "Calgary".to_string(),
        population: 1_200_000,
        city_history: "Calgary began as a fort called Fort Calgary that...".to_string(), // Assume this string is very, very long.
    };
    let canada_cities = CityData {
        names: vec![calgary.name], // This uses calgary.name, which is short
        histories: vec![calgary.city_history], // value moves here . . . this String is long.
    };
    println!("Calgary's history is: {}", calgary.city_history);
}

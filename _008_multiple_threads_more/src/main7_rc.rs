use std::rc::Rc;
#[derive(Debug)]
struct City {
    name: Rc<String>,
    population: u32,
    city_history: Rc<String>,
}
#[derive(Debug)]
struct CityData {
    names: Vec<Rc<String>>,
    histories: Vec<Rc<String>>,
}

fn main() {
    let calgary_name = Rc::new("Calgary".to_string());
    let calgary_history =
        Rc::new("Calgary began as a fort called Fort Calgary that...".to_string()); // +1
    let calgary = City {
        name: Rc::clone(&calgary_name),
        population: 1_200_000,
        city_history: Rc::clone(&calgary_history), // +1
    };
    let canada_cities = CityData {
        names: vec![Rc::clone(&calgary_name)],
        histories: vec![Rc::clone(&calgary_history)], // +1
    };
    println!("Calgary's history is: {}", calgary.city_history);
    println!("{}", Rc::strong_count(&calgary.city_history));

    let user_name = Rc::new(String::from("User MacUserson"));
    takes_a_string(Rc::clone(&user_name));
    takes_a_string(Rc::clone(&user_name));
}

fn takes_a_string(input: Rc<String>) {
    println!("It is: {input}")
}

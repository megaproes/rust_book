fn main() {
    let my_cities = ["Beirut", "Tel Aviv", "Nicosia"];
    for city in my_cities {
        println!("{city}");
    }
    for city in &my_cities {
        println!("{city}");
    }
    for city in my_cities.iter() {
        println!("{city}");
    }

    let my_cities = ["Beirut", "Tel Aviv", "Nicosia"];
    let [city1, _city2, _city3] = my_cities;
    println!("{city1}");

    let int_array = [1, 5, 9, 13, 17, 21, 25, 29];
    let string_array = int_array.map(|i| i.to_string());
    println!("{int_array:?}");
    println!("{string_array:?}");
}

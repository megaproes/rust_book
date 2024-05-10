// #[derive(Debug)]
// struct City {
//     name: &str,
//     date_founded: u32,
// }
#[derive(Debug)]
struct City {
    name: &'static str,
    date_founded: u32,
}
fn main() {
    let my_city = City {
        name: "Ichinomiya",
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);

    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];

//     let my_city = City {
//         name: &city_names[0], // the problem
//         date_founded: 1921,
//     };
}

// “The City struct has a lifeime that we will call 'a, and its 
// name property must also live at least as long as 'a.
// Other shorter lifetimes will not be accepted.”
#[derive(Debug)]
struct City<'a> {
    name: &'a str, // This means that it will only take a reference for name if it lives as long as City
    date_founded: u32,
}
// The 'a means “Please only take an input for name if it lives at least as long as City.” 
// It does not mean, “This will make the input for name live as long as City.”
fn main() {
    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];
    let mut my_city = City {
        name: &city_names[0],
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);
    
    {
        let temp_str = "sa".to_string();
        my_city.name = temp_str.as_str();// isnt valid
    }
    println!("{}", my_city.name)
}

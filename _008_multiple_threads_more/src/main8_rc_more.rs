use std::rc::Rc;

#[derive(Debug)]
struct City<'a> {
    name: &'a str,
    date_founded: u32,
}
#[derive(Debug)]
struct Country<'a> {
    cities: Vec<City<'a>>,
}
#[derive(Debug)]
struct World<'a> {
    countries: Vec<Country<'a>>, // That works just fine, but it took a lot of typing <'a>
}
fn main() {
    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];
    let my_city = City {
        name: &city_names[0],
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);

    //
    // rc
    let city_names = vec![
        Rc::new("Ichinomiya".to_string()),
        Rc::new("Kurume".to_string()),
    ];
    let my_city = City2 {
        name: Rc::clone(&city_names[0]),
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}

#[derive(Debug)] // with rc i don't put lifetimes
struct City2 {
    name: Rc<String>,
    date_founded: u32,
}
#[derive(Debug)]
struct Country2 {
    cities: Vec<City2>,
}
#[derive(Debug)]
struct World2 {
    countries: Vec<Country2>,
}
impl World2 {}

// replace() does a swap and then returns the other item—that’s all there is to it.
// In other words, .replace() replaces the value with what you put in and returns the old
// value, which makes it useful with a let binding to create a variable.

// Moves src into the referenced dest, returning the previous dest value.

// Neither value is dropped.

// If you want to replace the values of two variables, see swap.
// If you want to replace with a default value, see take.
use std::mem;
struct City {
    name: String,
}
impl City {
    fn change_name(&mut self, name: &str) {
        let former = mem::replace(&mut self.name, name.to_string());
        println!("{former} is now called {new}.", new = self.name);
    }
}
fn main() {
    let mut capital_city = City {
        name: "Constantinople".to_string(),
    };
    capital_city.change_name("Istanbul");

    let mut just_str = "suka_blyat";
    capital_city.change_name(&just_str);
    println!("{just_str}")
}

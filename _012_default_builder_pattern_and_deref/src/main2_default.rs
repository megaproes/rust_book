// So why implement Default instead of writing a new() or some other function? 
// Here are a few good reasons why you might want to  implement Default:

// 1. Default is a trait, so if you implement Default, you can pass your type into any-
// thing that requires it. Sometimes, you will come across functions or traits that
// require Default to be implemented, such as the .unwrap_or_default() method.

// 2. Your type might need to be a parameter in another struct or enum that wants to
// implement Default. To implement Default using #[derive(Default)], all of a
// type’s parameters need to implement it, too.

// 3. Having Default gives users of your types a general idea of how to use them. For
// example, you might want to have a method called new() or create() to make a
// type with lots of customization. But you could also implement Default so the
// user can just create one without thinking about all the settings.

// 4. Default is really convenient when working with parameters in a struct.

#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
}
#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain,
}
impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
            height: 170,
            weight: 70,
            lifestate: LifeState::Alive,
        }
    }
}
fn main() {
    let character_1 = Character::default();
    println!(
        "The character {:?} is {:?} years old.",
        character_1.name, character_1.age
    );
}

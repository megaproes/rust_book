// 3. Blanket Implementation for Reference Types

// Sometimes, you want to implement a trait for references to a type if the type itself implements a certain trait. Here's an example:

trait Describe {
    fn describe(&self) -> String;
}

impl<T: Describe> Describe for &T {
    fn describe(&self) -> String {
        (*self).describe()
    }
} //i can remove it and nothing happens

struct Person {
    name: String,
    age: u32,
}

impl Describe for Person {
    fn describe(&self) -> String {
        format!("Name: {}, Age: {}", self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let person_ref: &Person = &person;
    println!("{}", person_ref.describe());
}

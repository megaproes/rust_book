#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}
#[derive(Debug)]
enum Mood {
    Good,
    Bad,
    Sleepy,
}

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
    mood_type: Mood,
}
impl Animal {
    fn new_cat(age: u8, mood_type: Mood) -> Self {
        Self {
            age: age,
            animal_type: AnimalType::Cat,
		  mood_type: mood_type,
        }
    }
    fn new_dog(age: u8, mood_type: Mood) -> Self {
        Self {
            age: age,
            animal_type: AnimalType::Dog,
		  mood_type: mood_type,
        }
    }
    fn check_type(&self) {
        match self.animal_type {
            AnimalType::Dog => println!("The animal is a dog"),
            AnimalType::Cat => println!("The animal is a cat"),
        }
    }
    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed animal to dog! Now it's {self:?}");
    }
    fn change_to_cat(&mut self) {
        self.animal_type = AnimalType::Cat;
        println!("Changed animal to cat! Now it's {self:?}");
    }
}

fn main() {
    let mut new_animal = Animal::new_cat(10, Mood::Bad);
    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();
}

struct Animal {
    name: String,
}
trait DogLike {
    fn bark(&self);
    fn run(&self);
}
impl DogLike for Animal {
    fn bark(&self) {
        println!("{}, stop barking!!", self.name);
    }
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}
fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };
    rover.bark();
    rover.run();
}

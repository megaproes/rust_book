/// This is a struct that does nothing
pub struct DoesNothing {}
/// This struct only has one method.
pub struct PrintThing {}
impl PrintThing {
    /// This function just prints a message.
    pub fn prints_something() {
        println!("I am printing something");
    }
}
fn main() {}

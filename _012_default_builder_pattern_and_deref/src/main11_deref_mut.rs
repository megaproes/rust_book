use std::ops::{Deref, DerefMut};
struct HoldsANumber(u8);
impl HoldsANumber {
    fn prints_the_number_times_two(&self) {
        println!("{}", self.0 * 2);
    }
}
impl Deref for HoldsANumber {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for HoldsANumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
fn main() {
    let mut my_number = HoldsANumber(20);
    my_number.prints_the_number_times_two(); // Calls the method defined in HoldsANumber

    // Access inner methods directly using deref coercion
    println!("{:?}", my_number.checked_sub(100)); // Calls u8's checked_sub method
    *my_number = 30; // DerefMut allows this
    println!("{}", (*my_number).pow(2)); // Calls u8's pow method

    // Alternative without explicit dereferencing
    _ = my_number.pow(2);
}

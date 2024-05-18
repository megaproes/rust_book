struct HoldsANumber(u8);

use std::ops::Deref;
struct DerefExample<T> {
    value: T,
}
impl<T> Deref for DerefExample<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn main() {
    let boxed_number = Box::new(20);
    println!("This works fine: {}", *boxed_number);
    // let my_number = HoldsANumber(20);
    // println!("This fails though: {}", *my_number + 20); // my_number cannot be derefenced

    let x: DerefExample<char> = DerefExample { value: 'a' };
    assert_eq!('a', *x);
}

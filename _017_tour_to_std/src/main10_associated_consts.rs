trait HasNumbers {
    const SET_NUMBER: usize = 10;
    const EXTRA_NUMBER: usize; // This other const, however, is unknown. You have to choose its value when implementing this trait.

    // These two commented-out functions are similar in behavior to the consts. 
    // One has a default implementation, while the other only
    // shows the return type and has to be written  out by anyone implementing the trait.
    
    // fn set_number() -> usize { 10 }
    // fn extra_number() -> usize;
}
struct NothingSpecial;
impl HasNumbers for NothingSpecial {
    const EXTRA_NUMBER: usize = 10;
    // const SET_NUMBER: usize = 20; // If you uncommented this, the struct NothingSpecial would have a value of 20 for SET_NUMBER instead of 10
}
fn main() {
    print!("{} ", NothingSpecial::SET_NUMBER);
    print!("{}", NothingSpecial::EXTRA_NUMBER);
}

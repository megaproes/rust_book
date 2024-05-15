// There are a few differences and limitations when you use impl Trait compared to
// regular generics. One difference is that for impl Trait you can’t decide the type — the function decides it

use std::fmt::Display;
fn prints_it_impl_trait(input: impl Display) {
    println!("You can print many things, including {input}");
}
fn prints_it_regular_generic<T: Display>(input: T) {
    println!("You can print many things, including {input}");
}
fn main() {
    prints_it_regular_generic::<u8>(100);
    prints_it_impl_trait(100);
    prints_it_impl_trait(100u8);
    // prints_it_impl_trait::<u8>(100);
    
    // gives_higher(8, 10);
}

// this is wrong, because there's nothing to say about that 'one' and 'two' are of the same type, it just states that it accepts
// 
// fn gives_higher(one: impl PartialOrd + Display, two: impl PartialOrd + Display) {
//     let higher = if one > two { one } else { two };
//     println!("{higher} is higher.");
// }

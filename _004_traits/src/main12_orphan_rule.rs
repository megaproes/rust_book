// Implementing Your Trait on Your Type: This is always allowed.

// Your trait
trait MyTrait {
    fn do_something(&self);
}

// Your type
struct MyType;

// Implementing your trait on your type
impl MyTrait for MyType {
    fn do_something(&self) {
        println!("Doing something!");
    }
}

// Implementing Your Trait on Someone Else's Type: This is also allowed. It's how you extend functionality of types that you don't own.
// A trait from your codebase
trait MyTrait2 {
    fn do_something(&self);
}

// Implementing your trait on an external type, like `Vec<T>` from the Rust standard library
impl<T> MyTrait2 for Vec<T> {
    fn do_something(&self) {
        println!("The Vec is doing something!");
    }
}

// Implementing Someone Else's Trait on Your Type: You can implement external traits on types you define.

// Using `Display` trait from the standard library
use std::fmt;

// Your type
struct MyTyp2;

// Implementing `Display` trait on your type
impl fmt::Display for MyTyp2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyType Display implementation")
    }
}



// Implementing Someone Else's Trait on Someone Else's Type: This is where the orphan rule applies and is not allowed.
// This prevents multiple crates from implementing the same trait on the same type, which would lead to ambiguity and conflict.

// Trying to implement the `Display` trait from the standard library on `Vec<T>` which is also from the standard library
// This will not compile due to the orphan rule.
impl<T> fmt::Display for Vec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Error: the trait `std::fmt::Display` cannot be implemented for the type `std::vec::Vec<T>`
        // ...
    }
}

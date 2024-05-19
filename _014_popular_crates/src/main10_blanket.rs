// Why We Need Blanket Traits

// 1. Generic Behavior: Blanket traits enable you to define generic behavior that applies to a broad category of types.
// This reduces code duplication and makes the codebase more maintainable.
// 2. Code Reuse: They allow you to reuse existing functionality across multiple types, ensuring that the
// same logic does not need to be implemented repeatedly.
// 3. Abstraction: Blanket traits help in abstracting over different types
// that share common characteristics, making it easier to write generic and flexible code.

// 1. Implementing a Trait for All Types that Implement Another Trait
use std::fmt::{self, Display};

struct disp_struct {
    name: &'static str,
}
impl Display for disp_struct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
trait Printable {
    fn print(&self);
}

impl<T: Display> Printable for T {
    fn print(&self) {
        println!("{}", self);
    }
}

fn main() {
    let x = 42;
    let s = "Hello, world!";
    let temp = disp_struct { name: "str" };
    x.print();
    s.print();
    temp.print();
}

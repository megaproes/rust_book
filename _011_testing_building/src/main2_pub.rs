// 1. pub for a struct—pub makes the struct public, but the parameters are still private.

// To make a parameter public, you have to write pub for it, too. The same rule
// applies to tuple structs, too, so to make a pub Email(String) fully public, you
// would have to write pub Email(pub String). So a pub Email(String) is a type
// called Email which the user can use, but they can’t use .0 to access the String
// inside. (In the next chapter, we will learn about a popular trait called Deref that
// lets you use inner methods, like all the methods for String in this case, while
// keeping a type’s parameters private.)

// 2. pub for an enum or trait—Everything becomes public.

// For a trait, this means  every method in the trait, and for an enum, this means every variant of the
// enum. This makes sense because traits are about giving the same behavior to
// something. And enums are about choosing between variants, and you need to see them all to choose them.

// 3. pub for a module — A top-level module will be pub by default inside its own crate
// (as we saw in the previous example) but won’t be accessible from outside with-
// out pub. And modules inside modules all need pub to be public.

// The Rust reference (http://mng.bz/XqGa) sums this up quite well in a single sen-
// tence: “By default, everything is private, with two exceptions: items in a pub Trait are
// public by default; Enum variants in a pub enum are also public by default.”

mod print_things {
    #[derive(Debug)]
    pub struct Billy {
        name: String,
        pub times_to_print: u32,
    }
    impl Billy {
        pub fn new(times_to_print: u32) -> Self {
            Self {
                name: "Billy".to_string(),
                times_to_print,
            }
        }
        pub fn print_billy(&self) {
            for _ in 0..self.times_to_print {
                println!("{}", self.name);
            }
        }
    }
}
fn main() {
    use print_things::*; // imports everything from the module print_things. / glob operator.
    let my_billy = Billy::new(3);
    my_billy.print_billy();
}

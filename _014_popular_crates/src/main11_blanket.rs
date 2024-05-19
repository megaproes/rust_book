// Implementing a Trait for All Types that Implement Another Custom Trait
// Let's say we have a custom trait MyTrait and we want to implement another trait AnotherTrait for all types that implement MyTrait.

trait MyTrait {
    fn my_method(&self);
}

trait AnotherTrait {
    fn another_method(&self);
}

impl<T: MyTrait> AnotherTrait for T {
    fn another_method(&self) {
        self.my_method();
        println!("Another method called!");
    }
}

struct MyStruct;

impl MyTrait for MyStruct {
    fn my_method(&self) {
        println!("My method called!");
    }
}

fn main() {
    let my_struct = MyStruct;
    my_struct.another_method();
}

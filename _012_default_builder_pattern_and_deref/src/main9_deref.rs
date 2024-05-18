use std::ops::Deref;
#[derive(Debug)]
struct HoldsANumber(u8);
impl Deref for HoldsANumber {
    // this is the associated type — a type that goes together with a trait. The return value is Self::Target, which we decided will be a u8.
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        // Rust calls .deref() when you use * or use the dot operator when using a method.
        // We just defined Target as a u8, so this &Self::Target is
        // easy to understand: it’s a reference to a u8. If Self::Target is a u8, then &Self::Target is a &u8
        &self.0
    }
}

fn main() {
    let my_number = HoldsANumber(20);
    println!("{:?}", *my_number + 20);

    //     let sa = just_test("sa".to_string());
    //     println!("{}-{}", sa.some_func(), sa.some_func());
}
struct just_test(String);
impl just_test {
    fn some_func(self) -> String {
        self.0
    }
}

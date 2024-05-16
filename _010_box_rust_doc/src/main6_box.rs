use std::mem::size_of;
trait JustATrait {}
enum EnumOfNumbers {
    I8(i8),
    AnotherI8(i8),
    OneMoreI8(i8),
}
impl JustATrait for EnumOfNumbers {}
struct StructOfNumbers {
    an_i8: i8,
    another_i8: i8,
    one_more_i8: i8,
}
impl JustATrait for StructOfNumbers {}
enum EnumOfOtherTypes {
    I8(i8),
    AnotherI8(i8),
    Collection(Vec<String>),
}
impl JustATrait for EnumOfOtherTypes {}
struct StructOfOtherTypes {
    an_i8: i8,
    another_i8: i8,
    a_collection: Vec<String>,
}
impl JustATrait for StructOfOtherTypes {}
struct ArrayAndI8 {
    array: [i8; 1000],
    an_i8: i8,
    in_u8: u8,
}
impl JustATrait for ArrayAndI8 {}
fn main() {
    println!(
        "{}, {}, {}, {}, {}",
        size_of::<EnumOfNumbers>(),
        size_of::<StructOfNumbers>(),
        size_of::<EnumOfOtherTypes>(),
        size_of::<StructOfOtherTypes>(),
        size_of::<ArrayAndI8>(),
    );
}
fn returns_just_a_trait() -> JustATrait {
    let some_enum = EnumOfNumbers::I8(8);
    some_enum
}
fn returns_just_a_trait2() -> Box<dyn JustATrait> {
    let some_enum = EnumOfNumbers::I8(8);
    Box::new(some_enum)
}

// Box<T> is 8 bytes, Box<&T> (a reference) is also 8 bytes, but a Box<[T]>
// (a slice) is 16 bytes. Why is that? It’s because a slice can be any size (any
//     length), so the Box needs to store the length, too, and that takes 8 more
//     bytes. If it doesn’t need to know the length, it just stores the memory address,
//     and that’s just 8 bytes, not 16. In either case, the compiler knows the size and
//     will be happy with it.

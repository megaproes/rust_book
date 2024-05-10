trait HasSomeLifeTime<'a, 'b> {}
struct SomeStruct<'a, 'b> {
    name: &'a str,
    other: &'b str,
}
impl SomeStruct<'a, 'b> {}
impl<'a, 'b> HasSomeLifeTime<'a, 'b> for SomeStruct<'a, 'b> {}


struct some_struct<Tr>{name: Tr}
impl some_struct<T>{}

fn main() {
    
}
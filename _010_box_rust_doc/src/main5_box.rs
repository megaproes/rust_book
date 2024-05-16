// struct Holder {
//     next_holder: Option<Holder>,
// }

#[derive(Debug)]
struct Holder {
    next_holder: Option<Box<Holder>>,
}
fn main() {
    let x = Holder {
        next_holder: Some(Box::new(Holder {
            next_holder: Some(Box::new(Holder { next_holder: None })),
        })),
    };
    println!("{x:#?}");
}

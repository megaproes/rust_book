use std::collections::BTreeMap;

#[derive(Debug)]
struct One;
#[derive(Debug)]
struct Two;
impl From<One> for Two {
    fn from(one: One) -> Self {
        Two
    }
}
fn main() {
    let two: Two = One.into();
    let try_two = Two::try_from(One);
    println!("{two:?}, {try_two:?}");

    let my_btree_map = BTreeMap::from([
        ("customer_1_money".to_string(), 10),
        ("customer_2_money".to_string(), 200),
    ]);
}

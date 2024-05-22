use std::time::{SystemTime, UNIX_EPOCH};
fn timestamp() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}
fn send_data_to_user() {} // This function is empty, but pretend that it sends the users of our system some data in case it comes across as true.
fn main() {
    let true_false = (true, false);
    println!("{} {}", true_false.0 as u8, true_false.1 as i32);

    let true_false: (i128, u16) = (true.into(), false.into());
    println!("{} {}", true_false.0, true_false.1);

    // .then() and .then_some() turn a bool into an Option. With .then(), you write a closure, and the closure is
    // called if the item is true. Whatever is returned from the closure gets wrapped in an Option.
    let (tru, fals) = (true.then(|| 8), false.then(|| 8));
    println!("{:?}, {:?}", tru, fals);

    let bool_vec = vec![true, false, true, false, false];
    let result_vec = bool_vec
        .into_iter()
        .enumerate()
        .map(|(index, b)| {
            b.then(|| {
                let timestamp = timestamp();
                send_data_to_user();
                timestamp
            })
            .ok_or_else(|| {
                let time = timestamp();
                format!("Error with item {index} at {time}")
            })
        })
        .collect::<Vec<_>>();
    println!("{result_vec:#?}");
}

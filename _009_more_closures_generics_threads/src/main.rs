fn do_something<F>(f: F)
where
    F: FnOnce(),
{
    f();
}
fn do_something2<F>(f: F)
where
    F: Fn(),
{
    f();
}
fn main() {
    let some_vec = vec![9, 8, 10];
    do_something(|| {
        some_vec
            .into_iter()
            .for_each(|x| println!("The number is: {x}"));
    });
    // println!("{some_vec:?}"); // invalid

    let some_vec = vec![9, 8, 10];
    do_something2(|| {
        some_vec.iter().for_each(|x| println!("The number is: {x}"));
    });
    do_something2(|| {
        some_vec.iter().for_each(|x| println!("The number is: {x}"));
    }); // valid

    //
    //  something weird is happening
    let some_vec = vec![9, 8, 10];
    weird_function(|| {
        some_vec.iter().for_each(|x| println!("The number is: {x}"));
    });
    weird_function(|| {
        some_vec.iter().for_each(|x| println!("The number is: {x}"));
    }); // valid, but why? doesnt weird_function owned the some_vec?
}

fn weird_function<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

// One interesting fact about closures is that one closure is never the same type as
// another closure, even if the signature is the same. The types are always different
// because Fn, FnMut, and FnOnce are traits, not concrete types.

fn takes_a_closure_and_does_nothing<F>(f: F)
where
    F: Fn() -> i32,
{
}
fn takes_two_closures_and_does_nothing<F>(first: F, second: F)
where
    F: Fn() -> i32,
{
}
fn main() {
    let my_closure = || 9;
    takes_a_closure_and_does_nothing(my_closure);
    my_closure();

    let first_closure = || 9;
    let second_closure = || 9;
    takes_two_closures_and_does_nothing(first_closure, second_closure); // a problem

    takes_two_closures_and_does_nothing2(first_closure, second_closure); // correct
}

fn takes_two_closures_and_does_nothing2<F, G>(first: F, second: G)
where
    F: Fn() -> i32,
    G: Fn() -> i32,
{
}

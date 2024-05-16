fn just_takes_a_variable<T>(item: T) {}
fn main() {
    let my_number = 1;
    just_takes_a_variable(my_number);
    just_takes_a_variable(my_number);

    let my_box = Box::new(1);
    just_takes_a_variable(my_box.clone());
    just_takes_a_variable(my_box);

    let my_box = Box::new(1);
    let an_integer = *my_box;
}

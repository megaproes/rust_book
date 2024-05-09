fn main() {
    let mut number_iter = [7, 8, 9, 10].into_iter();
    let first_two = number_iter.take(2).collect::<Vec<_>>();
    //let second_two = number_iter.take(2).collect::<Vec<_>>(); // wrong

    let mut number_iter = [7, 8, 9, 10].into_iter();
    let first_two = number_iter.by_ref().take(2).collect::<Vec<_>>();
    let second_two = number_iter.take(2).collect::<Vec<_>>();
}

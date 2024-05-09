use std::collections::VecDeque;

fn main() {
    let mut my_vec = vec![0; 600_000];
    for _ in 0..600000 {
        my_vec.remove(0);
    } // very long since it will shrift to the left all time!

    let mut my_vec = VecDeque::from(vec![0; 600000]);
    for i in 0..600000 {
        my_vec.pop_front();
    }
}

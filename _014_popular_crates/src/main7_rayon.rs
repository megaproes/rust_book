// Rayon is a popular crate that lets you speed up your Rust code by automatically spawn-
// ing multiple threads when working with iterators and related types. Instead of using
// thread::spawn() to spawn threads, you can just add par_ to the iterator methods you already know.
use rayon::prelude::*;
use std::thread::available_parallelism;
fn main() {
    let mut my_vec = vec![0; 2_000_000];
    my_vec
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, number)| *number += index + 1);

    println!("Estimated parallelism on this computer: {:?}", available_parallelism()
    );
    
    let mut without_rayon = vec![];
    let mut with_rayon = vec![];
    for _ in 0..10 {
        let mut my_vec = vec![0; 2_000_000];
        let now = std::time::Instant::now();
        my_vec.iter_mut().enumerate().for_each(|(index, number)| {
            *number += index + 1;
            *number -= index + 1;
        });
        let elapsed = now.elapsed();
        without_rayon.push(elapsed.as_micros());
	   
	   
        let mut my_vec = vec![0; 2_000_000];
        let now = std::time::Instant::now();
        my_vec
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, number)| {
                *number += index + 1;
                *number -= index + 1;
            });
        let elapsed = now.elapsed();
        with_rayon.push(elapsed.as_micros());
    }
    println!(
        "Average time without rayon: {} microseconds",
        without_rayon.into_iter().sum::<u128>() / 10
    );
    println!(
        "Average time with rayon: {} microseconds",
        with_rayon.into_iter().sum::<u128>() / 10
    );
}

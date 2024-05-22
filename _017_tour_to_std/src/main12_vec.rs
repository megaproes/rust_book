fn main() {
    let mut my_vec = vec![100, 90, 80, 0, 0, 0, 0, 0];
    my_vec.sort();
    println!("{:?}", my_vec);

    // .dedup() means “de-duplicate.” It will remove items that are the same in a vector, but only if they are next to each other.
    // This next code will not just print "sun", "moon":
    let mut my_vec = vec!["sun", "sun", "moon", "moon", "sun", "moon", "moon"];
    my_vec.dedup();
    println!("{:?}", my_vec);

    // So, if you want to use .dedup() to remove every duplicate, just .sort() first:
    let mut my_vec = vec!["sun", "sun", "moon", "moon", "sun", "moon", "moon"];
    my_vec.sort();
    my_vec.dedup();
    println!("{:?}", my_vec);

    //     You can split a Vec with .split_at(), while .split_at_mut() lets you do the same
    // if you need to change the values. These give you two slices while leaving the original Vec intact:
    let mut big_vec = vec![0; 6]; // six zeros,
    println!("{big_vec:?}");
    let (first, second) = big_vec.split_at_mut(3);
    std::thread::scope(|s| {
        s.spawn(|| {
            for num in first {
                *num += 1;
            }
        });
        s.spawn(|| {
            for num in second {
                *num -= 5;
            }
        });
    });
    println!("{big_vec:?}");

    //The .drain() method lets you pull a range of values out of a Vec, giving you an
    // iterator. This iterator keeps a mutable borrow on the original Vec so doing something
    // like collecting it into another Vec or outright using the drop() method will let you
    // access the original Vec again:
    let mut original_vec = ('A'..'K').collect::<Vec<_>>();
    println!("{original_vec:?}");
   
    let drain = original_vec.drain(2..=5);
    println!("\nPulled these chars out: {drain:?}");
    drop(drain);
    
    println!("Here's what's left: {original_vec:?}");
    let drain_two = original_vec.drain(2..=4).collect::<Vec<_>>();
    println!("Original vec: {original_vec:?}\nSecond drain: {drain_two:?}");
}

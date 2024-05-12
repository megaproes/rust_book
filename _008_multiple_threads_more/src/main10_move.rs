fn main() {
    let mut join_handles = vec![];
    for num in 0..10 {
        let handle = std::thread::spawn(move || { // without move the problem
            println!("Inside thread number: {num}");
        });
        join_handles.push(handle);
    }
    for handle in join_handles {
        handle.join().unwrap();
    }
}

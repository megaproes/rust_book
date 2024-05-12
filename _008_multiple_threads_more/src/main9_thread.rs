fn main() {
    //     for _ in 0..=10 {
    //         std::thread::spawn(|| {
    //             println!("I am printing something");
    //         });
    //     } // bad

    for _ in 0..10 {
        let handle = std::thread::spawn(|| {
            println!("I am printing something");
        });
        handle.join();
    } // but it will create thread by once and not really concurently
    
    // instead we want this:
    let mut join_handles = vec![];
    for _ in 0..10 {
        let handle = std::thread::spawn(|| {
            println!("I am printing something");
        });
        join_handles.push(handle);
    }
    for handle in join_handles {
        handle.join().unwrap();
    }
    
    
}

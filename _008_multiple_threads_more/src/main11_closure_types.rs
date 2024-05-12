fn main() {
    // 1. FnOnce — Takes by value
    // 2. FnMut — Takes a mutable reference
    // 3. Fn — Takes a regular reference
    {
        // Fn
        let my_string = String::from("I will go into the closure (as FnOnce)");
        let my_closure = || println!("{my_string}");
        my_closure();
        my_closure();
    }
    {
        // FnMut
        let mut my_string = String::from("I will be changed in the closure (as FnMut)");
        let mut my_closure = || {
            my_string.push_str(" now");
            println!("{my_string}");
        };
        my_closure();
        my_closure();
    }
    {
        // FnOnce
        let my_vec: Vec<i32> = vec![8, 9, 10];
        let my_closure = || {
            my_vec.iter().for_each(|item| println!("{item}"));
        };
        my_closure();
	   
	   // This won’t work because the closure, FnOnce, took ownership of my_vec, and my_vec is already gone
        // my_closure();
    }
}

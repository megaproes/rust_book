fn takes_fn<F: Fn()>(f: F) {
    f();
    f();
}

fn takes_fnmut<F: FnMut()>(mut f: F) {
    f();
    f();
}

fn takes_fnonce<F: FnOnce()>(f: F) {
    f();
    // f();
}

fn main() {
    let mut my_string = String::from("Hello there");
    let prints_string = || {
        println!("{my_string}");
    };
    takes_fn(prints_string);
    
    let adds_exclamation_and_prints = || {
        my_string.push('!');
        println!("{my_string}");
    };
    takes_fnmut(adds_exclamation_and_prints);
    
    let prints_then_drops = || {
        println!("Now dropping {my_string}");
        drop(my_string);
    };
    takes_fnonce(prints_then_drops);
    // takes_fnonce(prints_then_drops);
    
    // or:
    {
        let mut my_string = String::from("Hello there");
        takes_fn(|| {
            println!("{my_string}");
        });
        takes_fnmut(|| {
            my_string.push('!');
            println!("{my_string}");
        });
        takes_fnonce(|| {
            println!("Now dropping {my_string}");
            drop(my_string);
        });
    }
}

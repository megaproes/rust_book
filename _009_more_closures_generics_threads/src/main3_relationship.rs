use std::{thread, time::Duration};

// pub trait Fn: FnMut 		-- Fn needs to implement FnMut first
// pub trait FnMut: FnOnce	-- FnMut needs to implement FnOnce first
// pub trait FnOnce			-- FnOnce needs nothing to implement first
// That means that all closures implement FnOnce
// The trait after the : is known as a supertrait. FnOnce is a supertrait of FnMut, and FnMut is a supertrait of Fn.
fn takes_fnonce<F: FnOnce()>(f: F) {
    f();
}
fn takes_fnmut<F: FnMut()>(mut f: F) {
    f();
    f();
}
fn main() {
    let mut my_string = String::from("Hello there");
    let prints_string = || {
        println!("{my_string}");
    };
    takes_fnonce(prints_string);
    takes_fnmut(prints_string);
    
    let adds_exclamation_and_prints = || {
        my_string.push_str("!!!!!!!!!!!");
        println!("{my_string}");
    };
    takes_fnonce(adds_exclamation_and_prints);
    println!("{}", my_string);
    thread::sleep(Duration::from_secs(500));
    
    
    let prints_then_drops = || {
        println!("Now dropping {my_string}");
        drop(my_string);
    };
    takes_fnonce(prints_then_drops);
}
// This is why you sometimes see books say that Fn is the “most powerful” of the three
// closure traits because it can be passed in no matter which closure trait is written. At
// the same time, having a function that takes an Fn is the most restrictive because an Fn
// closure must implement all three traits. No FnMut or FnOnce can be an argument in a
// function that wants an Fn.

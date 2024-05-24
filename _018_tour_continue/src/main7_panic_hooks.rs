fn main() {
    std::panic::set_hook(Box::new(|info| {
        if let Some(payload) = info.payload().downcast_ref::<&str>() {
            println!("{payload}");
        } else {
            println!("No payload!");
        }
    }));
    panic!("Oh no");
}

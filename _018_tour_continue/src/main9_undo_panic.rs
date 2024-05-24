fn main() {
    std::panic::set_hook(Box::new(|_| {
        println!("Something went wrong / 문제가 생겼습니다!");
    }));
    let _ = std::panic::take_hook();
    panic!();
}

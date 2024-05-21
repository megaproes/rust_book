fn main() {
    for (mut key, mut value) in std::env::vars() {
        key.push('!');
        value.push('!');
        std::env::set_var(key, value);
    }
    for (key, value) in std::env::vars() {
        println!("{key}: {value}");
    }
}

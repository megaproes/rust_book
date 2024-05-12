use std::borrow::Cow;
#[derive(Debug)]
struct ErrorInfo {
    error: LocalError,
    message: String,
}
#[derive(Debug)]
enum LocalError {
    TooBig,
    TooSmall,
}
fn generate_message(message: &'static str, error_info: Option<ErrorInfo>) -> Cow<'static, str> {
    match error_info {
        None => message.into(),
        Some(info) => format!("{message}: {info:?}").into(),
    }
}
fn main() {
    let msg1 = generate_message("Everything is fine", None);
    let msg2 = generate_message(
        "Got an error",
        Some(ErrorInfo {
            error: LocalError::TooBig,
            message: "It was too big".to_string(),
        }),
    );
    for msg in [msg1, msg2] {
        match msg {
            Cow::Borrowed(msg) => {
                println!("Borrowed, didn't need an allocation:\n {msg}")
            }
            Cow::Owned(msg) => {
                println!("Owned, because we needed an allocation:\n {msg}")
            }
        }
    }
    let mut input = "Hello, Rust!";
    let modified = maybe_modify(input);
    println!("{}", modified);
    
    input = "s";
}

fn maybe_modify(input: &str) -> Cow<str> {
    if input.starts_with("Hello") {
        Cow::Borrowed(input)
    } else {
        Cow::Owned(input.to_uppercase())
    }
}















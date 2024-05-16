use std::error::Error;
use std::fmt;
use std::sync::mpsc::RecvError;
enum MyError {
    TooMuchStuff,
    CantConnect,
    NoUserRegistered,
    SomethingElse,
}
impl std::error::Error for MyError {}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Wouldn't you like to know...")
    }
}
impl fmt::Debug for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lol not telling you what went wrong")
            .finish()
    }
}
fn give_error_back(is_tru: bool) -> Box<dyn Error> {
    if is_true {
        Box::new(MyError::TooMuchStuff)
    } else {
        Box::new(RecvError)
    }
}

fn main() {
    let errs = [true, false, false, true]
        .into_iter()
        .map(|boolean| give_error_back(boolean))
        .collect::<Vec<_>>();

    println!("{errs:#?}");
    for err in errs.iter() {
        if let Some(my_error) = err.downcast_ref::<MyError>() {
            println!("Got a MyError!");
        } else if let Some(parse_error) = err.downcast_ref::<RecvError>() {
            println!("Got a RecvError!");
        }
    }
}

use std::cell::RefCell;
#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
}
fn main() {
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };
    println!("{:?}", user_1.active);

    let user_1: User = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };
    let mut borrow: std::cell::RefMut<bool> = user_1.active.borrow_mut();
    *borrow = false;

    // double borrow_mut will panic
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };

    let borrow_one = user_1.active.borrow_mut();
    let borrow_two = user_1.active.borrow_mut(); // not OK

    // There are two ways to be sure that your code won’t panic when using a RefCell:
    // 1. Always immediately change the value with .borrow_mut() without assigning this to a variable. 
    // If no variables are holding on to the output of .borrow_mut(), there is no way the code will panic.
    // ---------------------------------------------------
    // 2. Use the .try_borrow_mut() method instead of borrow_mut() if there is a
    // chance of a double borrow. This will return an error if the RefCell is already borrowed.
    
    
}

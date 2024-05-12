// Okay, first I need a book struct.
// Nothing in there yet - will add later
struct Book;
// A book can be hardcover or softcover, so add an enum…
enum BookType {
    HardCover,
    SoftCover,
}
// should take a &Book and return an Option<String>
fn get_book(book: &Book) -> Option<String> {
    // todo!();
}
// should take a ref Book and return a Result...
fn delete_book(book: &Book) -> Result<(), String> {
    // todo!();
}
// TODO: impl block and make these functions methods…
// TODO: make this a proper error
fn check_book_type(book_type: &BookType) {
    // Let's make sure the match statement works
    match book_type {
        BookType::HardCover => println!("It's hardcover"),
        BookType::SoftCover => println!("It's softcover"),
    }
}
fn main() {
    let book_type = BookType::HardCover;
    // Okay, let's check this function!
    check_book_type(&book_type);
}

struct Book2 {
    name: String,
    year: u8,
}
fn make_book() -> Book2 {
    Book2 {
        name: todo!(),
        year: todo!(),
    }
}

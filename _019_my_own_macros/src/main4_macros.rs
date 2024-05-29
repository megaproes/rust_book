//  To give a macro more than one item at a
// time, we have to use a different syntax. Instead of $input, it will be $($input1),*. The
// * means “zero or more,” while the comma before the * means that the tokens have to
// be separated by commas. If you want to match on one or more tokens, use + instead of *.

macro_rules! print_anything {
	($($input1:tt),*) => { // If we used + instead of *, it would give an error because one time we gave it no input. So * is a bit more flexible
	let output = stringify!($($input1),*);
	println!("{}", output);
	};
}

macro_rules! print_anything2 {
	($($input1:tt);*) => {
	let output = stringify!($($input1),*);
	println!("{}", output);
	};
}
fn main() {
    print_anything!(ththdoetd, rcofe);
    print_anything!();
    print_anything!(87575oehq75onth, ntohe, 987987o, 097);

    print_anything2!(ththdoetd; rcofe);
    print_anything2!();
    print_anything2!(87575oehq75onth; ntohe; 987987o; 097);
}

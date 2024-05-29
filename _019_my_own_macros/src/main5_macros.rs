macro_rules! make_a_function {
	($name:ident, $($input:tt),+) => {
		fn $name() {
		let output = stringify!($($input),+);
		println!("{}", output);
		}
	};
}
fn main() {
    make_a_function!(print_it, 5, 5, 6, I);
    print_it();
    
    make_a_function!(say_its_nice, this, is, really, nice);
    say_its_nice();
}

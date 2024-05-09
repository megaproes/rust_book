fn print_it<T: AsRef<str>>(input: T) {
	//let some_input = input.as_ref(); 
	println!("{}", input.as_ref());
 }
 
 fn main() {
	let s = String::from("Hello, String!");
	let s_slice: &str = "Hello, &str!";
	let s_ref: &String = &s.clone();
	
	// All these types can be converted to a &str reference
	print_it(s);
	print_it(s_slice);
	print_it(s_ref);
 }
 
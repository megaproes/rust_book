//  you can tell a macro that it will receive an expression, a type name, an identifier, and so on.
// Here is a simple example of a macro that expects an expression:
macro_rules! might_print {
    ($input:expr) => {
        println!("You gave me: {}", $input); // The $input:expr part is important. With this, you can
                                             // give the macro any expression, which can then be used inside the macro code block
                                             // with any name we choose, which, in this case, we decided to call $input. In macros,
                                             // variables (technically, they are called arguments) start with a $. In this macro, if you
                                             // give it one expression, it will print it.
    };
}
macro_rules! might_print2 {
    ($input:expr) => {
        println!("You gave me: {:?}", $input);
    };
}
macro_rules! wants_expression {
    // We can see that the macro is parsing as expected if we tell it to expect an expression but give it a statement:
    ($input:expr) => {
        println!("You matched the macro input!");
    };
}

macro_rules! wants_statement {
    ($input:stmt) => { //  the macro to expect a statement.
        println!("You matched the macro input!");
    };
}
fn main() {
    might_print!(6);

    might_print2!(());
    might_print2!(6);
    might_print2!(vec![8, 9, 7, 10]);

    // wants_expression!(let x = 9);
    wants_statement!(let x = 9);
}

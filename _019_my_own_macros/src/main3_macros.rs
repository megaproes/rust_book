// So what can a macro see besides expr and stmt? Here is the full list—give it a read, but
// don’t worry about memorizing it:
// block | expr | ident | item | lifetime | literal | meta | pat | path | stmt | tt | ty | vis. -- fragment specifiers.

//  block—A block expression inside {}
//  expr—An expression
//  ident—An identifier, such as a variable name
//  item—A struct, module, etc.
//  lifetime—'a, 'static, etc.
//  literal—"hello", 9, etc
//  meta—The information that goes inside attributes
//  pat—A path (like std::vec::Vec)
//  stmt—A statement (like let x = 9), without the semicolon
//  tt—A token tree, which matches almost anything
//  ty—A type name
//  vis—A visibility modifier like pub

// However, for most macros, you will probably use expr, ident, and tt. tt means token tree, which sort of means any type of input.
macro_rules! check {
    ($input1:ident, $input2:expr) => {
        println!(
            "Is {:?} equal to {:?}? {:?}",
            $input1,
            $input2,
            $input1 == $input2
        );
    };
}

macro_rules! print_anything {
    ($input:tt) => {
        let output = stringify!($input);
        println!("{}", output);
    };
}
fn main() {
    let x = 6;
    let my_vec = vec![7, 8, 9];

    check!(x, 6);
    check!(my_vec, vec![7, 8, 9]);
    check!(x, 10);

    print_anything!(ththdoetd);
    print_anything!(87575oehq75onth);
}

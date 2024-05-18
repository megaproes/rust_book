// const fn is a  function that is “permitted to call from a const context” and adds that in this case “the
// function is interpreted by the compiler at compile time” (http://mng.bz/g78v). 
// Note the word permitted in the wording: a const fn doesn’t have to be called during compile
// time, but it always can be. So a const fn can be called anywhere, not just in const con-
// texts. As the reference states, “you can freely do anything with a const function that
// you can do with a regular function.”

const NUMBER: u8 = give_eight();
const fn give_eight() -> u8 {
    8
}
fn main() {
    let mut my_vec = Vec::new();
    my_vec.push(give_eight());
}

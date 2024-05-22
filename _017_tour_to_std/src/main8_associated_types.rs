trait ChangeForm {
    type SomethingElse;
    fn change_form(self) -> Self::SomethingElse;
}
impl ChangeForm for String {
    type SomethingElse = char;
    fn change_form(self) -> Self::SomethingElse {
        self.chars().next().unwrap_or('-')
    }
}
impl ChangeForm for i32 {
    type SomethingElse = i64;
    fn change_form(self) -> Self::SomethingElse {
        println!("i32 just got really big!");
        i64::MAX
    }
}
fn main() {
    let string1 = "".to_string();
    println!("{}", string1.change_form());
    let string2 = "I'm back!".to_string();
    println!("{}", String::change_form(string2));
    let small_num = 1;
    println!("{}", small_num.change_form());
    let also_small_num = 0;
    println!("{}", i32::change_form(also_small_num));
}

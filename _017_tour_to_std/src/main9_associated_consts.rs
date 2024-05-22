struct SizeTenString(String);
impl SizeTenString {
    const SIZE: usize = 5;
}
fn main() {
    println!("{}", SizeTenString::SIZE);

    println!("{:?}", SizeTenString2::try_from("This one's long"));
    println!("{:?}", SizeTenString2::try_from("Too short"));
    println!("{:?}", SizeTenString2::try_from("Just right"));
}



#[derive(Debug)]
struct SizeTenString2(String);
impl SizeTenString2 {
    const SIZE: usize = 10;
}
impl TryFrom<&'static str> for SizeTenString2 {
    type Error = String;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if input.chars().count() == Self::SIZE {
            Ok(Self(input.to_string()))
        } else {
            Err(format!("Length must be {} characters!", Self::SIZE))
        }
    }
}

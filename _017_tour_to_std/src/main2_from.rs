#[derive(Debug)]
enum Hours {
    Working(u32),
    NotWorking(u32),
    Error(u32),
}
impl From<u32> for Hours {
    fn from(value: u32) -> Self {
        match value {
            hour if (8..17).contains(&hour) => Hours::Working(value),
            hour if (0..=24).contains(&hour) => Hours::NotWorking(value),
            wrong_hour => Hours::Error(wrong_hour),
        }
    }
}
fn main() {
    let int_array = [1, 5, 9, 13, 17, 21, 25, 29];
    let hours_array = int_array.map(Hours::from);
    println!("{hours_array:?}");
}

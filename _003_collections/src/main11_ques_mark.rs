use std::{num::ParseIntError, u16};
fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    //     let parsed_number = input
    //         .parse::<u16>()?
    //         .to_string()
    //         .parse::<u32>()?
    //         .to_string()
    //         .parse::<i32>()?;

    //     println!("Number parsed successfully into {parsed_number}");
    //     Ok(parsed_number)

    let parsed_number = match input.parse::<u16>() {
        Ok(value) => match value.to_string().parse::<u32>() {
            Ok(value) => match value.to_string().parse::<i32>() {
                Ok(value) => Ok(value),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    };

    if let Ok(number) = parsed_number {
        println!("Number parsed successfully into {number}");
    }

    parsed_number
}
fn main() {
    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_str(item);
        println!("{parsed:?}");
    }
}

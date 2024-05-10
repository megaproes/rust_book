fn main() {
    for num in ["9", "nine", "ninety-nine", "9.9"]
        .into_iter()
        .map(|num| num.parse::<f32>()).flatten()
    {
        println!("{num:?}");
    }// without flatten it prints wrapped values within Ok/Err like Ok(9.0)/Err(ParseFloatError { kind: Invalid })
    
    // With flatten it remove one outer iterators!
}

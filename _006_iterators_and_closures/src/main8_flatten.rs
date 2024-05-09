fn main() {
    for num in ["9", "nine", "ninety-nine", "9.9"]
        .into_iter()
        .map(|num| num.parse::<f32>()).flatten()
    {
        println!("{num:?}");
    }
}

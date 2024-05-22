fn four_operations(input: f64) {
    println!(
        "For the number {}:
	floor: {}
	ceiling: {}
	rounded: {}
	truncated: {}\n",
        input,
        input.floor(),
        input.ceil(),
        input.round(),
        input.trunc() //  .trunc()—Cuts off the part after the period. Truncate means “to cut off.”
    );
}
fn main() {
    four_operations(9.1);
    four_operations(100.7);
    four_operations(-1.1);
    four_operations(-19.9);

    let nums = vec![8.0_f64, 7.6, 9.4, 10.0, 22.0, 77.345, -7.77, -10.0];
    
    let max = nums
        .iter()
        .fold(f64::MIN, |num, next_num| num.max(*next_num));
	
    let min = nums
        .iter()
        .fold(f64::MAX, |num, next_num| num.min(*next_num));
	
    println!("{max}, {min}");
}

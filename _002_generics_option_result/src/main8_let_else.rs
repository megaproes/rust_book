fn main() {
    let my_vec = vec![2, 3, 4];
    for index in 0..10 {
        if let Some(number) = my_vec.get(index) {
            println!("The number is: {number}");
        }
	   
        let Some(number) = my_vec.get(index) else {
            continue;
        };
	   
        println!("The number is: {number}"); // it's skipped if 'else' above is constructed when there's 'None' returned.
    }
}

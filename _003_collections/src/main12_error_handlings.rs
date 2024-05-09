fn get_fourth(input: &Vec<i32>) -> i32 {
    let fourth = input.get(3).expect("Input vector needs at least 4 items");
    *fourth
}
fn main() {
    let my_vec = vec![9, 0, 10];
    let fourth = get_fourth(&my_vec);

    {
        let vector = vec![None, Some(1000)];
        try_two_unwraps(vector);
    }
    {
        let my_vec = vec![8, 9, 10];
        let fourth = my_vec.get(3).unwrap_or(&0); // always sets the value
        println!("{fourth}");
    }
}

// Better with 'expect' because gives a message
fn try_two_unwraps(input: Vec<Option<i32>>) {
    println!(
        "Index 0 is: {}",
        input[0].expect("The first unwrap had a None!")
    );
    println!(
        "Index 1 is: {}",
        input[1].expect("The second unwrap had a None!")
    );
}

// fn try_two_unwraps(input: Vec<Option<i32>>) {
//     println!("Index 0 is: {}", input[0].unwrap());
//     println!("Index 1 is: {}", input[1].unwrap());
// }

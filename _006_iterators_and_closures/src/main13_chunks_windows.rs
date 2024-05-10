fn main() {
    let num_vec = vec![1, 2, 3, 4, 5, 6, 7];
    for chunk in num_vec.chunks(3) {
        println!("{:?}", chunk);
    }
    println!();
    for window in num_vec.windows(3) {
        println!("{:?}", window);
    }
    
    print!("\n");
    for chunk in num_vec.chunks_exact(3) {
        println!("{:?}", chunk);
    }
}

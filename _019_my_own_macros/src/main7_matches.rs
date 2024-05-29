const ALLOWS_TRUE: bool = false;

fn main() {
    println!("{}", matches!(9, 9));
    println!("{}", matches!(9, 0..=10));
    println!("{}", matches!(9, 100..=1000));

    println!("{}", matches!(9, 9,));
    println!("{}", matches!(9, 0..=10,));
    println!("{}", matches!(9, 100..=1000,));
    
    println!("{}", matches!(9, 9 if ALLOWS_TRUE));
}

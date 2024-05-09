fn main() {
    let mut new_vec: Vec<i32> = Vec::new();
    let mut counter = 1;
    loop {
        new_vec.push(counter);
        counter += 1;
        if counter == 10 {
            break;
        }
    }
    println!("{new_vec:?}");

    // or much shorter:
    let new_vec: Vec<i32> = (1..).take(10).collect::<Vec<i32>>();
    println!("{new_vec:?}");

    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let new_vec = my_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();
    println!("{new_vec:?}");
    // println!("{my_vec:?}");
    
    
    let new_vec: Vec<i32> = (0..).take(100).collect::<Vec<i32>>();
    let mut temp_test1 =new_vec.iter().take(100); 
    assert_eq!(temp_test1.next(), Some(&0));
    assert_eq!(temp_test1.clone().skip(9).next(), Some(&10));
    assert_eq!(temp_test1.clone().skip(18).next(), Some(&19));
    assert_eq!(temp_test1.clone().skip(36).take(4).next(), Some(&37))
}

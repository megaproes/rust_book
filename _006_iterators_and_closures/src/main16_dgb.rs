fn main() {
    let my_number = 8;
    dbg!(my_number);

    let mut my_number = dbg!(9);
    dbg!(my_number += 10);
    let new_vec = dbg!(vec![8, 9, 10]);
    let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>());
    dbg!(double_vec);

    let new_vec = vec![8, 9, 10];
    let double_vec = new_vec
        .iter()
        .inspect(|first_item| println!("The item is: {first_item}"))
        .map(|x| x * 2)
        .inspect(|next_item| println!("Then it is: {next_item}"))
        .collect::<Vec<i32>>();

    let new_vec = vec![8, 9, 10];
    let double_vec = new_vec
        .iter()
        .inspect(|first_item| {
            println!("The item is: {first_item}");
            match **first_item % 2 {
                0 => println!("It is even."),
                _ => println!("It is odd."),
            }
            println!("In binary it is {:b}.", first_item);
        })
        .map(|x| x * 2)
        .collect::<Vec<i32>>();

    let a = [1, 4, 2, 3];

    // this iterator sequence is complex.
    let sum = a
        .iter()
        .cloned()
        .filter(|x| x % 2 == 0)
        .fold(0, |sum, i| sum + i);

    // let's add some inspect() calls to investigate what's happening
    let sum = a
        .iter()
        .cloned()
        .inspect(|x| println!("about to filter: {x}"))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("made it through filter: {x}"))
        .fold(0, |sum, i| sum + i);

    println!("{sum}");

    // cloned
    let a = [1, 2, 3];

    let v_cloned: Vec<_> = a.iter().cloned().collect();

    // cloned is the same as .map(|&x| x), for integers
    let v_map: Vec<_> = a.iter().map(|&x| x).collect();

    assert_eq!(v_cloned, vec![1, 2, 3]);
    assert_eq!(v_map, vec![1, 2, 3]);
    //  To get the best performance, try to clone late:

    let a = [vec![0_u8, 1, 2], vec![3, 4], vec![23]];
    // don't do this:
    let slower: Vec<_> = a.iter().cloned().filter(|s| s.len() == 1).collect();
    assert_eq!(&[vec![23]], &slower[..]);
    // instead call `cloned` late
    let faster: Vec<_> = a.iter().filter(|s| s.len() == 1).cloned().collect();
    assert_eq!(&[vec![23]], &faster[..]);

    let a = [0, 1, 2];
    let mut iter: std::slice::Iter<i32> = a.iter();
    let temp = iter.filter(|x: &&i32| **x < 0).filter(|x: &&i32| **x < 0);
    
    let a = [0, 1, 2];
    let mut iter: std::array::IntoIter<i32, 3> = a.into_iter();
    let temp = iter.filter(|x: &i32| *x < 0);
    
    
    
    
    
}

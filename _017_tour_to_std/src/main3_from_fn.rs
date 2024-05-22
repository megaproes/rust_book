// 1. When using from_fn() for an array, you can pull in the index of each item if
// you want to use it or use |_| if you don’t need it.

// 2. Most of the time, you will have to tell the compiler the length of the array.

// 3. If you are comparing one array to another, you won’t need to tell the compiler
// the length. But you might want to write out the length anyway for the benefit of  anyone else reading your code.

fn main() {
    let array = std::array::from_fn(|i| i); // it knows the size since of type inference, it looks just below and sees
    assert_eq!(array, [0, 1, 2, 3, 4]);

    // We could take the index for the array we are creating, but we don’t care about it.
    let array = std::array::from_fn(|_| "Don't care about the index");
    assert_eq!(
        array,
        [
            "Don't care about the index",
            "Don't care about the index",
            "Don't care about the index",
            "Don't care about the index",
            "Don't care about the index"
        ]
    );

    let array: [_; 5] = std::array::from_fn(|_| "Don't need the index");
    let array: [&str; 5] = std::array::from_fn(|_| "Don't need the index");
}

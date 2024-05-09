fn main() {
    let some_str = "Er ist noch nicht erklärt. Aber es gibt Krieg. Verlaß dich drauf.";
    for (index, item) in some_str.match_indices(|c| c > 'z') {
        println!("{item} at {index}");
    }
    for (index, item) in some_str.match_indices(". ") {
        println!("'{item}' at index {index}");
    }
}

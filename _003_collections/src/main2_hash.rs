use std::collections::HashMap;
fn main() {
    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the World");
    println!("{:?}", book_hashmap.get(&1));

    {
        let mut book_hashmap: HashMap<i32, &str> = HashMap::new();
        book_hashmap.insert(1, "L'Allemagne Moderne");
        let key = 1;
        match book_hashmap.get(&key) {
            Some(val) => println!("Key {key} has a value already: {val}"),
            None => {
                book_hashmap.insert(key, "Le Petit Prince");
            }
        }
        println!("{:?}", book_hashmap.get(&1));
    }
}

use std::collections::{btree_map::Entry, HashMap};
fn main() {
    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "Eye of the World",
        "Eye of the World",
    ];
    let mut book_hashmap = HashMap::new();
    for book in book_collection {
        book_hashmap.entry(book).or_insert(true);
    }
    for (book, true_or_false) in book_hashmap {
        println!("Do we have {book}? {true_or_false}");
    }

    {
        let mut scores = HashMap::new();

        // Using entry to insert a key with its value and update it
        let team = "Blue";
        let score = scores.entry(team).or_insert(50);
        *score += 10;

        println!("Score for {team}: {score}");
    }

    {
        let book_collection = vec![
            "L'Allemagne Moderne",
            "Le Petit Prince",
            "Eye of the World",
            "Eye of the World",
        ];
        let mut book_hashmap = HashMap::new();
        for book in book_collection {
            let return_value = book_hashmap.entry(book).or_insert(0);
            *return_value += 1;
        }
        for (book, number) in book_hashmap {
            println!("{book}, {number}");
        }
    }

    {
        let mut scores: HashMap<&str, i32> = HashMap::new();
        scores.insert("value", 1);
        let entry = scores.entry("non");
        println!("{:?}", entry); // Ok
        let some_mut = entry.or_insert(100); // this will not insert since we have 1 as value
                                             // println!("{:?}", entry); // Not ok, since or_insert takes ownership . . .

        // println!("{:?}", entry);
    }
}

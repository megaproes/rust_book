use std::collections::HashMap;
fn main() {
    let data = vec![
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10),
    ];

    let mut survey_hash: HashMap<&str, Vec<i32>> = HashMap::new();
    for item in data {
        let entr: &mut Vec<i32> = survey_hash.entry(item.0).or_insert(Vec::new());
	   entr.push(item.1);
        // survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);
    }
    assert_eq!(survey_hash.keys().count(), 2);
    for (male_or_female, numbers) in survey_hash {
        println!("{male_or_female}: {numbers:?}");
    }
}

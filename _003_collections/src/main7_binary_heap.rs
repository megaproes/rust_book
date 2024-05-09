use std::collections::BinaryHeap;
fn main() {
    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];
    let mut heap = BinaryHeap::new();
    for num in many_numbers {
        heap.push(num);
    }
    println!("First item is largest, others are out of order: {heap:?}");
    while let Some(num) = heap.pop() {
        println!("Popped off {num}. Remaining numbers are: {heap:?}");
    }

    {
        let mut jobs: BinaryHeap<(i32, &str)> = BinaryHeap::new();
        jobs.push((100, "Reply to email from the CEO"));
        jobs.push((80, "Finish the report today"));
        jobs.push((5, "Watch some YouTube"));
        jobs.push((70, "Tell your team members thanks for always working hard"));
        jobs.push((30, "Plan who to hire next for the team"));
        while let Some((_, job)) = jobs.pop() {
            println!("You need to: {}", job);
        }
    }
}

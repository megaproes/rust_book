#[derive(Debug)]
struct CombinedEvents {
    num_of_events: u32,
    data: Vec<String>,
}
fn main() {
    let events = [
        "Went to grocery store",
        "Came home",
        "Fed cat",
        "Fed cat again",
    ];
    let empty_events = CombinedEvents {
        num_of_events: 0,
        data: vec![],
    };
    let combined_events = events
        .iter()
        .fold(empty_events, |mut total_events, next_event| {
            total_events.num_of_events += 1;
            total_events.data.push(next_event.to_string());
            total_events
        });
    println!("{combined_events:#?}");
}

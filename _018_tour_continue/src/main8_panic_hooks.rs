// Here, we are running some sort of software
// that accesses a database. We want to make sure that the database gets shut down prop-
// erly even if there is a panic, before the stack unwinds and the program stops. We’ll
// also add some pretend types and functions that demonstrate how our system is working and what sometimes goes wrong:

use rand::Rng;
struct Database {
    data: Vec<String>,
}
fn get_hour() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=30) // Uh oh, someone made a mistake, and
                          // sometimes the hour of the day is greater
                          // than 24. We’ll represent this with a simple
                          // function that returns a number up to 30.
}

fn shut_down_database(hour: u32) -> Result<(), String> {
    match hour {
        h if (6..18).contains(&h) => Ok(()),
        h if h > 24 => Err(format!("Internal error: hour {h} shouldn'texist")),
        h => Err(format!("Hour {h} is not working hours, can't shut down")),
    }
}
fn main() {
    std::panic::set_hook(Box::new(|info| {
        println!("Something went wrong / 문제가 생겼습니다!");
        println!("Panic info: {info}");
        let hour = get_hour();
        match shut_down_database(hour) {
            Ok(()) => println!("Shutting down database at {hour} o'clock!"),
            Err(e) => println!("Couldn't shut down database before panic finished: {e}"),
        }
    }));
    
    
    let mut db = Database { data: vec![] };
    db.data.push("Some data".to_string());
    panic!("Database broke");
}

fn main() {
    let numbers = vec![Some(10), None, Some(20), Some(30), None, Some(40)];

    // Process the vector with while let
    let mut iter = numbers.iter();
    while let Some(option) = iter.next() {
        // Use if let to extract the value if it exists
        if let Some(value) = option {
            println!("Found value: {}", value);
        } else {
            println!("Found a None");
        }
    }

    // Simulating a function that might fail
    let result = simulate_error_prone_operation(50);
    
    // Use let else to handle the Result
    let value = if let Ok(v) = result {
        v
    } else {
        // Default or error handling block
        println!("An error occurred, using default value.");
        0
    };

    println!("The processed value is: {}", value);
}

// A function returning a Result; might succeed or fail based on input
fn simulate_error_prone_operation(input: i32) -> Result<i32, String> {
    if input > 25 {
        Ok(input * 2)
    } else {
        Err("Input is too low".to_string())
    }
}

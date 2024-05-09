fn main() {
    let try_1 = [Some("Okay!"), None, Some("Okay!"), Some("Okay!"), None];
    let try_2 = [
        None,
        Some("Okay!"),
        Some("Okay!"),
        Some("Okay!"),
        Some("Okay!"),
    ];
    let try_3 = [
        Some("Okay!"),
        Some("Okay!"),
        Some("Okay!"),
        Some("Okay!"),
        None,
    ];
    for i in 0..try_1.len() {
        println!("{:?}", try_1[i].and(try_2[i]).and(try_3[i]));
    }
    
    // and:
    let x: Option<i32> = Some(5);
    let y = x.and(Some(10)); // y = Some(10)
    let z: Option<i32> = x.and(None); // z = None

    let x: Result<i32, &str> = Ok(5);
    let y = x.and(Ok(10)); // y = Ok(10)
    let z: Result<i32, &str> = x.and(Err("error")); // z = Err("error")
    
    // and_then:
    let x: Option<i32> = Some(5);
    let y = x.and_then(|val| Some(val + 5)); // y = Some(10)
    let z: Option<i32> = x.and_then(|val| None); // z = None

    let x: Result<i32, &str> = Ok(5);
    let y = x.and_then(|val| Ok(val + 5)); // y = Ok(10)
    let z: Result<i32, &str> = x.and_then(|val| Err("error")); // z = Err("error")
    
    
}

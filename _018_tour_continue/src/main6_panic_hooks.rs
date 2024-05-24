fn main() {
    // We can see that when a panic happens, it first tells us which thread panicked, why it
    // panicked, any error info, the location in the code where the panic happened, and a
    // note about how to display a backtrace:
    //     let try_parse = "my_num".parse::<u32>();
    //     println!("Error output: {try_parse:?}");
    //     let my_num = try_parse.unwrap();

    // So, that’s the default behavior. But we can change all of this if we want by using a
    // method called set_hook(). This sets up a global panic hook, which will be called
    // instead of the default panic hook. Inside this method is a closure in which we can do
    // whatever we like when a panic happens.

//     std::panic::set_hook(Box::new(|_| {
//         println!("Oops, that didn't work.");
//         println!("앗 뭔가 잘못 됐네요.");
//     }));
//     panic!(); // The panic hook does exactly what we told it to do, and even the location and error information is gone.

    // Where does the default information in a panic message come from? It would be nice if we could display that as well.
     std::panic::set_hook(Box::new(|info| {
        println!("Well, that didn't work: {info}");
    }));
    panic!();
    
    
}

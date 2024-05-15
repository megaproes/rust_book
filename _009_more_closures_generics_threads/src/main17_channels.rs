use std::sync::mpsc::channel;
fn main() {
    let (sender, receiver) = channel();
    drop(receiver);
    if let Err(e) = sender.send(5) {
        println!("Got an error: {e}")
    }

    // program will never end:
    {
        let (sender, receiver) = channel();
        sender.send(5).unwrap();
        sender.send(5).unwrap();
        println!("{:?}", receiver.recv());
        println!("{:?}", receiver.recv());
        println!("{:?}", receiver.recv()); // here the third call causes
    }
    
    // will end:
    {
        let (sender, receiver) = channel();
        sender.send(5).unwrap();
        sender.send(5).unwrap();
        drop(sender); // ok
        println!("{:?}", receiver.recv());
        println!("{:?}", receiver.recv());
        println!("{:?}", receiver.recv()); // ok
    }
}

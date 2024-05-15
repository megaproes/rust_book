use std::sync::mpsc::channel;
use std::thread;

fn main() {
    {
        let (sender, receiver) = channel();
        let sender_clone = sender.clone();

        thread::spawn(move || {
            sender.send("Message from first thread").unwrap();
        });

        thread::spawn(move || {
            sender_clone.send("Message from second thread").unwrap();
        });

        for received in receiver.iter().take(2) {
            println!("Received: {}", received);
        }
    }

    {
        let (sender, receiver) = channel();

        thread::spawn(move || {
            sender.send("Message 1").unwrap();
            sender.send("Message 2").unwrap();
            // Dropping the sender to close the channel
        });

        while let Ok(message) = receiver.recv() {
            println!("Received: {}", message);
        }

        println!("Channel closed");
    }
}

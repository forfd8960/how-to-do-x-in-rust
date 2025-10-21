use std::sync::mpsc;
use std::thread;

pub fn mpsc1() {
    // Create a channel for String messages
    let (tx, rx) = mpsc::channel::<String>();

    // Clone the sender for multiple producers (optional)
    let tx_clone = tx.clone();

    // Spawn a thread for the receiver
    let handle = thread::spawn(move || {
        // Receive and process messages
        while let Ok(msg) = rx.recv() {
            println!("Received: {}", msg);
        }
        println!("Channel closed.");
    });

    // Sender 1: Send from main thread
    tx.send("Hello from sender 1!".to_string()).unwrap();

    // Sender 2: Send from another thread
    let sender2 = thread::spawn(move || {
        tx_clone.send("Hello from sender 2!".to_string()).unwrap();
    });

    // Wait for senders to finish
    sender2.join().unwrap();
    drop(tx); // signal the send is end

    handle.join().unwrap();
}

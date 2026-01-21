use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn non_block_rev() {
    let (tx, rx) = mpsc::channel::<String>();

    // Send a message
    tx.send("Non-blocking message".to_string()).unwrap();

    // Try to receive (succeeds immediately)
    match rx.try_recv() {
        Ok(msg) => println!("Got: {}", msg),
        Err(mpsc::TryRecvError::Empty) => println!("No message yet"),
        Err(mpsc::TryRecvError::Disconnected) => println!("Sender disconnected"),
    }

    // Simulate no message
    thread::sleep(Duration::from_millis(10));
    match rx.try_recv() {
        Ok(_) => println!("Unexpected message"),
        Err(mpsc::TryRecvError::Empty) => println!("Empty, as expected"),
        Err(_) => println!("Disconnected"),
    }
}

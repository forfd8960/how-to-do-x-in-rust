use std::sync::mpsc;
use std::thread;

pub fn mpsc2() {
    let (tx, rx) = mpsc::channel::<i32>();

    // Create multiple sender threads
    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            // Simulate work
            thread::sleep(std::time::Duration::from_millis(100 * i as u64));
            tx_clone.send(i * 10).unwrap();
            println!("Sender {} sent {}", i, i * 10);
        });
    }

    // Drop the original tx to signal end (optional, but good practice)
    drop(tx);

    // Receiver in main thread
    for received in rx {
        println!("Received: {}", received);
    }
}

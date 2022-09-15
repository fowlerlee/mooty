use std::sync::mpsc::{channel, Sender, Receiver};
use std::{sync::mpsc, thread};

fn channels(){
    const N: i32 = 10;
    let (tx,rx): (Sender<i32>, Receiver<i32>) = channel();
    let handles = (0..N).map(|i| {
        let _tx = tx.clone();
        thread::spawn(move || {
            let _ = _tx.send(i).unwrap();
        })
    });
    for h in handles {
        h.join().unwrap();
    }
}

fn main() {
    println!("Hello, world!");
}

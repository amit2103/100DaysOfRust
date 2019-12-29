use std::thread;
use std::sync::mpsc;

fn main() {
    let handle = thread::spawn(|| {
        println!("Here's a Hello World from a thread");
    });

    handle.join().unwrap();

    println!("And a Welcome back from main");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi message from thread");
        tx.send(val).unwrap();
    });

    //this will block till message
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

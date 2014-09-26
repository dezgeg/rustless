use std::{os,io};
use std::io::BufferedReader;

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    spawn(proc() {
        let mut stream = io::stdin();
        for r in stream.lines() {
            match r {
                Err(e) => {
                    println!("Input/output error: {}", e);
                }
                Ok(line) => {
                    tx.send(line);
                }
            }
        }
    });

    loop {
        let s = rx.recv();
        print!("LINE: {}", s);
    }
}

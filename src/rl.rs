#![feature(globs)]
extern crate termutils;

use std::{os,io};
use std::io::BufferedReader;
use termutils::hexes::*;

fn main() {
    let (line_tx, line_rx): (Sender<String>, Receiver<String>) = channel();

    spawn(proc() {
        let mut stream = io::stdin();
        for r in stream.lines() {
            match r {
                Err(e) => {
                    println!("Input/output error: {}", e);
                }
                Ok(line) => {
                    line_tx.send(line);
                }
            }
        }
    });

    let (_, rows) = termutils::ios::size();
    let term = Term::new();
    let mut term_reader = term.r;
    let mut term_writer = term.w;
    let (line_tx, keypress_rx): (Sender<Keypress>, Receiver<Keypress>) = channel();
    spawn(proc() {
        loop {
            match term_reader.read() {
                None => break,
                Some(k) => line_tx.send(k),
            }
        }
    });

    loop {
        select!(
            line = line_rx.recv() => {
                term_writer.write("LINE: ");
                term_writer.write(line.as_slice());
                term_writer.flush();
            },
            key = keypress_rx.recv() => {
                term_writer.write("KEY: ");
                term_writer.write(format!("{}", key).as_slice());
                term_writer.write("\n");
                term_writer.flush();
            }
        );
    }
}

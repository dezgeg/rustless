#![feature(globs)]
extern crate termutils;

use std::{io};
use std::vec::{Vec};
use termutils::hexes::*;
use std::time::duration;

fn doit(line_rx: Receiver<String>, keypress_rx: Receiver<Keypress>, term_writer: &mut TermWriter) {
    let (_, term_rows) = termutils::ios::size();

    // State variables:
    let mut lines_printed = 0u;
    let mut topmost_line = 0u;
    let mut lines : Vec<String> = Vec::new();

    loop {
        select!(
            mut line = line_rx.recv() => {
                line.pop();
                lines.push(line.clone());
                lines_printed += 1;

                if lines_printed <= term_rows {
                    term_writer.write(line.as_slice());
                    if lines_printed < term_rows {
                        term_writer.write("\n");
                    }
                    term_writer.flush();
                }

            },
            key = keypress_rx.recv() => {
                if lines_printed < term_rows  {
                    continue
                }
                match key {
                    KeyUp => {
                        if topmost_line > 0 {
                            topmost_line -= 1;
                            term_writer.move_cursor(0, 0);
                            term_writer.scroll_reverse();
                            term_writer.write(lines[topmost_line].as_slice());
                            term_writer.flush();
                        }
                    }
                    KeyDown => {
                        if topmost_line + term_rows < lines.len() {
                            topmost_line += 1;
                            term_writer.move_cursor(0, term_rows - 1);
                            term_writer.scroll_forward();
                            term_writer.write(lines[topmost_line + term_rows - 1].as_slice());
                            term_writer.flush();
                        }
                    }
                    _ => (),
                }
            }
        );
    }
}

fn main() {
    termutils::ios::preserve(|| {
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
            io::timer::sleep(duration::MAX); // XXX FIXME, just block the task
        });

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

        // term_writer.cursor(true); // TODO
        doit(line_rx, keypress_rx, &mut term_writer);
    });
}

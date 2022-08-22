use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let mut f = File::open("Cargo.toml").unwrap();

    let mut target = Vec::new();

    let (tx, rx) = mpsc::channel();

    let mut n = 0;

    thread::spawn(move || {
        while n < 6 {
            tx.send(1).unwrap();
            n += 1;
            println!("{:?}", 0);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for r in rx {
        target.push(r);
        println!("{:?}", target);
    }

    println!("start");
}

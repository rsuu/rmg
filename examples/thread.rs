use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let mut f = File::open("Cargo.toml").unwrap();

    let mut target = String::new();

    let (tx, rx) = mpsc::channel();

    let mut n = 0;

    thread::spawn(move || {
        while n < 6 {
            let mut buffer = vec![0; 10];
            // read up to 10 bytes
            let line_read = f.read(&mut buffer[..]).unwrap();

            if line_read == 0 {
                break;
            }

            let content_chunks = str::from_utf8(&buffer[..line_read]).unwrap();

            tx.send(content_chunks.to_lowercase()).unwrap();
            println!("{}", n);
            n += 1;

            thread::sleep(Duration::from_secs(1));
        }
    });

    for r in rx {
        target.push_str(&r);
    }

    println!("{}", target);
}

use std::time::Duration;

fn main() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    const N: usize = 3;

    let data_mutex = Arc::new(Mutex::new(vec![]));
    let data_mutex_clone = Arc::clone(&data_mutex);

    let mut threads = Vec::new();

    threads.push(thread::spawn(move || {
        for f in 0..10 {
            let mut data = data_mutex_clone.lock().unwrap();
            data.push(1);
            drop(data);
            println!("{:?}", 0);
            std::thread::sleep(Duration::from_millis(1000));
        }
    }));

    println!("{:?}", 1);

    for f in threads.into_iter() {
        f.join().unwrap();
        println!("{:?}", *data_mutex.lock().unwrap());
    }
}

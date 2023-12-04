use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let num = Arc::new(Mutex::new(5));
    // allow `num` to be shared across threads (Arc) and modified
    // (Mutex) safely without a data race.

    let num_clone = num.clone();
    // create a cloned reference before moving `num` into the thread.

    thread::spawn(move || {
        loop {
            *num.lock().unwrap() += 1;
            // modify the number.
            thread::sleep(Duration::from_secs(10));
        }
    });

    output(num_clone);
}

fn output(num: Arc<Mutex<i32>>) {
    loop {
        println!("{:?}", *num.lock().unwrap());
        // read the number.
        //  - lock(): obtains a mutable reference; may fail,
        //    thus return a Result
        //  - unwrap(): ignore the error and get the real
        //    reference / cause panic on error.
        thread::sleep(Duration::from_secs(5));
    }
}


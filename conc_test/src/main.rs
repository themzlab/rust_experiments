use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a shared state (boolean flag) and a condition variable, 
    // wrapped in an Arc for safe sharing across threads
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    
    // Spawn two threads
    for i in 1..=4 {
        let pair_clone = pair.clone();
        thread::spawn(move || {

            // Clone the Arc to pass a reference to the shared state to the threads
            let (lock, cvar) = &*pair_clone;

            let mut started = lock.lock().unwrap();

            loop {
                // Clone the Arc to pass a reference to the shared state to the threads
                // let (lock, cvar) = &*pair_clone;

                // Lock the mutex to access the shared state
                // let mut started = lock.lock().unwrap();

                
                // println!("Thread {}: Message flag {}", i, started);

                started = cvar.wait(started).unwrap();

                // Reset the flag
                *started = false;

                // Print a message
                println!("Thread {}: Message flag {}", i, started);

            }
        });
    }

    thread::sleep(Duration::from_millis(100));
    // Loop in the main thread to signal every second
    let (lock, cvar) = &*pair;
    // let mut started = lock.lock().unwrap();
    // let mut started = lock.lock();

    for _ in 0..10 {
        thread::sleep(Duration::from_millis(2));

        // Signal the condition variable
        {
            // let (lock, cvar) = &*pair;
            let mut started = lock.lock().unwrap();
            // started = lock.lock().unwrap();
            *started = true;
            cvar.notify_all();
            println!("  end notify");
        }
    }
}

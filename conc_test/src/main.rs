use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a shared state (boolean flag) and a condition variable, 
    // wrapped in an Arc for safe sharing across threads
    let pair: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
    
    // Spawn a bunch of  threads
    for i in 1..=5 {
        let pair_clone: Arc<(Mutex<bool>, Condvar)> = pair.clone();
        thread::spawn(move || {

            // Clone the Arc to pass a reference to the shared state to the threads
            let (my_mutex, my_conditional_variable) = &*pair_clone;

            let mut started: std::sync::MutexGuard<'_, bool> = my_mutex.lock().unwrap();

            loop {

                started = my_conditional_variable.wait(started).unwrap();

                println!("Thread {}: Message flag {}", i, started);
                // Reset the flag, but only one thread will get to to this, the value is
                *started = false;

            }
        });
    }

    thread::sleep(Duration::from_millis(100));
    // Loop in the main thread to signal every second
    let (my_mutex, cvar) = &*pair;

    for i in 0..10 {
        thread::sleep(Duration::from_millis(2));

        // Signal the condition variable
        // When the block is entered, the mutex is locked, and the shared state (started) is 
        // accessed and modified
        {
            let mut started: std::sync::MutexGuard<'_, bool> = my_mutex.lock().unwrap();

            *started = true;
            // this seems to be a mechanism to notify the other threads that the value of started has changed
            cvar.notify_all();
            println!("\nend notify {} started= {}\n", i, started)
        }
        // the other threads will not advance even though the notification has been sent
        // until the code block goes out of scope and releases my_mutex
        // the lock guard returned by lock() goes out of scope. This automatically releases
        // the lock on the mutex
    }
    
    thread::sleep(Duration::from_millis(100))
}

use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a shared state (boolean flag) and a condition variable, 
    // wrapped in an Arc for safe sharing across threads
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    
    // Spawn Four threads
    for i in 1..=4 {
        let pair_clone = pair.clone();
        thread::spawn(move || {

            // Clone the Arc to pass a reference to the shared state to the threads
            let (my_mutex, my_conditional_variable) = &*pair_clone;

            let mut started = my_mutex.lock().unwrap();

            loop {

                started = my_conditional_variable.wait(started).unwrap();
            
                // Reset the flag
                *started = false;

                // Print a message
                println!("Thread {}: Message flag {}", i, started);

            }
        });
    }

    thread::sleep(Duration::from_millis(100));
    // Loop in the main thread to signal every second
    let (my_mutex, cvar) = &*pair;

    for i in 0..10 {
        thread::sleep(Duration::from_millis(2));

        // Signal the condition variable
        {
            let mut started = my_mutex.lock().unwrap();

            *started = true;
            cvar.notify_all();
            println!("  end notify {}", i);
        }
    }
    thread::sleep(Duration::from_millis(100))
}

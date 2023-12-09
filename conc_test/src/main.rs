use std::sync::{Arc, Mutex, Condvar};
use std::{time::Duration, sync::atomic::{AtomicBool, Ordering}};
use std::thread;

fn main() {
    // Create a shared state (boolean flag) and a condition variable, 
    // wrapped in an Arc for safe sharing across threads
    let pair: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
    let running = Arc::new(AtomicBool::new(true));

    let pair_clone: Arc<(Mutex<bool>, Condvar)> = pair.clone();
    let my_running = running.clone();

    // make just one super thread
    thread::spawn(move || {

        // Clone the Arc to pass a reference to the shared state to the threads
        let (my_mutex, my_conditional_variable) = &*pair_clone;
        let mut i = 1;
        loop {
                {
                    let mut started: std::sync::MutexGuard<'_, bool> = my_mutex.lock().unwrap();

                    i = 1;
                    let my_interval = Duration::from_millis(50);
                    {
                        started = my_conditional_variable.wait(started).unwrap();
                        *started = false;
                    }
                    
                    loop {
                        if !my_running.load(Ordering::Relaxed) {
                            break;
                        }
                        i = i + 1;
                        println!("content 1:  {}", i);
                        thread::sleep(my_interval);
                    }
                    // why doesn't the following line ever run
                    println!("content 1: is complete");
                }
                my_running.store(true, Ordering::Relaxed);
                println!{"go to the second loop"};
                // second
                {
                    let mut started: std::sync::MutexGuard<'_, bool> = my_mutex.lock().unwrap();

                    i=1;
                    let my_interval = Duration::from_millis(50);
                    {
                        started = my_conditional_variable.wait(started).unwrap();
                        *started = false;
                    }
                    loop {
                        if !my_running.load(Ordering::Relaxed) {
                            break;
                        }
                        i = i + 1;
                        println!("CONTENT 2:  {}, {}", i, started);
                        thread::sleep(my_interval);
                    }
                    println!("CONTENT 2: is complete");

                }
                my_running.store(true, Ordering::Relaxed);

            }
    });

    thread::sleep(Duration::from_millis(100));
    // Loop in the main thread to signal every second
    let (my_mutex, cvar) = &*pair;

    
    thread::sleep(Duration::from_millis(20));

    // Signal the condition variable
    // When the block is entered, the mutex is locked, and the shared state (started) is 
    // accessed and modified
    {
        println!("the code gets to kick off the first set of DATA");
        let mut started: std::sync::MutexGuard<'_, bool> = my_mutex.lock().unwrap();
        
        // dereference operator. access the data (bool) that the MutexGuard is pointing to
        thread::sleep(Duration::from_millis(1000));
        *started = true;

        // this seems to be a mechanism to notify the other threads that the value of started has changed
        cvar.notify_all();
        println!("\nend notify started= {}\n", started);
    }
    // the other threads will not advance even though the notification has been sent
    // until the code block goes out of scope and releases my_mutex
    // the lock guard returned by lock() goes out of scope. This automatically releases
    // the lock on the mutex
    //}
    thread::sleep(Duration::from_millis(2000));
    println!("ENDING notifications for content 1\n\n");
    
    running.store(false, Ordering::SeqCst);
    thread::sleep(Duration::from_millis(1000));
    println!("now kick into the SECOND loop\n\n");
    {
        let mut started: std::sync::MutexGuard<'_, bool> = my_mutex.lock().unwrap();
        
        // dereference operator. access the data (bool) that the MutexGuard is pointing to
        *started = true;

        // this seems to be a mechanism to notify the other threads that the value of started has changed
        cvar.notify_all();
        println!("\nend notify SECOND started= {}\n", started)
    }
    thread::sleep(Duration::from_millis(1000));
    println!("now kick into the FIRST data again\n\n");
    running.store(false, Ordering::SeqCst);
    thread::sleep(Duration::from_millis(2000));
    {
        let mut started: std::sync::MutexGuard<'_, bool> = my_mutex.lock().unwrap();
        
        // dereference operator. access the data (bool) that the MutexGuard is pointing to
        *started = true;

        // this seems to be a mechanism to notify the other threads that the value of started has changed
        cvar.notify_all();
        println!("\nend notify FIRST started= {}\n", started)
    }
    thread::sleep(Duration::from_millis(3000));

//     for i in 0..6 {
//         thread::sleep(Duration::from_millis(2));

//         // Signal the condition variable
//         // When the block is entered, the mutex is locked, and the shared state (started) is 
//         // accessed and modified
//         {
//             let mut started: std::sync::MutexGuard<'_, bool> = my_mutex.lock().unwrap();
            
//             // dereference operator. access the data (bool) that the MutexGuard is pointing to
//             *started = true;

//             // this seems to be a mechanism to notify the other threads that the value of started has changed
//             cvar.notify_all();
//             println!("\nend notify SECOND {} started= {}\n", i, started)
//         }
//         // the other threads will not advance even though the notification has been sent
//         // until the code block goes out of scope and releases my_mutex
//         // the lock guard returned by lock() goes out of scope. This automatically releases
//         // the lock on the mutex
//     }
//     thread::sleep(Duration::from_millis(100))
}

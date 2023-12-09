use std::sync::{Arc, Mutex, Condvar};
use std::{time::Duration, sync::atomic::{AtomicBool, Ordering}};
use std::thread;

fn main() {
    // Create a shared state (boolean flag) and a condition variable, 
    // wrapped in an Arc for safe sharing across threads
    let pair1: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
    
    let running1: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    let running2: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));

    let pair1_clone: Arc<(Mutex<bool>, Condvar)> = pair1.clone();
    let pair2_clone: Arc<(Mutex<bool>, Condvar)> = pair2.clone();

    let my_running_1: Arc<AtomicBool> = running1.clone();
    let my_running_2: Arc<AtomicBool> = running2.clone();

    // make just one super thread
    thread::spawn(move || {

        // Clone the Arc to pass a reference to the shared state to the threads
        let (my_mutex1, my_conditional_variable_1) = &*pair1_clone;
        let (my_mutex2, my_conditional_variable_2) = &*pair2_clone;
        loop {
                {
                    

                    let mut i = 1;
                    let my_interval = Duration::from_millis(50);
                    
                    {
                        let mut started1: std::sync::MutexGuard<'_, bool> = my_mutex1.lock().unwrap();    
                        started1 = my_conditional_variable_1.wait(started1).unwrap();
                        *started1 = false;
                    }
                    
                    loop {
                        if !my_running_1.load(Ordering::SeqCst) {
                            break;
                        }
                        i = i + 1;
                        println!("THREAD:content 1:  {}", i);
                        thread::sleep(my_interval);
                    }
                    // why doesn't the following line ever run
                    println!("THREAD:content 1: is complete");
                }
                my_running_1.store(true, Ordering::SeqCst);
                println!{"THREAD:go to the second loop"};
                // second
                {
                    let mut i = 1;
                    let my_interval = Duration::from_millis(50);
                    
               
                    {
                        let mut started2: std::sync::MutexGuard<'_, bool> = my_mutex2.lock().unwrap();    
                        started2 = my_conditional_variable_2.wait(started2).unwrap();
                        *started2 = false;
                    }
                    loop {
                        if !my_running_2.load(Ordering::SeqCst) {
                            break;
                        }
                        i = i + 1;
                        println!("THREAD:CONTENT 2:  {}", i);
                        thread::sleep(my_interval);
                    }
                    println!("THREAD:CONTENT 2: is complete");

                }
                my_running_2.store(true, Ordering::SeqCst);

            }
    });

    thread::sleep(Duration::from_millis(100));
    // Loop in the main thread to signal every second
    let (my_mutex1, cvar1) = &*pair1;
    let (my_mutex2, cvar2) = &*pair2;
    // Signal the condition variable
    // When the block is entered, the mutex is locked, and the shared state (started) is 
    // accessed and modified
    {
        println!("the code gets to kick off the first set of DATA");
        let mut started_1: std::sync::MutexGuard<'_, bool> = my_mutex1.lock().unwrap();
        
        // dereference operator. access the data (bool) that the MutexGuard is pointing to
        thread::sleep(Duration::from_millis(1000));
        *started_1 = true;

        // this seems to be a mechanism to notify the other threads that the value of started has changed
        cvar1.notify_all();
        println!("\nend notify started= {}\n", started_1);
    }
    // the other threads will not advance even though the notification has been sent
    // until the code block goes out of scope and releases my_mutex
    // the lock guard returned by lock() goes out of scope. This automatically releases
    // the lock on the mutex
    //}
    thread::sleep(Duration::from_millis(3000));
    println!("ENDING notifications for content 1\n\n");
    running1.store(false, Ordering::SeqCst);
    // 
    thread::sleep(Duration::from_millis(50));
    println!("now kick into the SECOND loop\n\n");
    {
        let mut started2: std::sync::MutexGuard<'_, bool> = my_mutex2.lock().unwrap();
        
        // dereference operator. access the data (bool) that the MutexGuard is pointing to
        *started2 = true;

        // this seems to be a mechanism to notify the other threads that the value of started has changed
        cvar2.notify_all();
        println!("\nend notify SECOND started= {}\n", started2)
    }
    thread::sleep(Duration::from_millis(3000));
    running2.store(false, Ordering::SeqCst);
    
    println!("now kick into the FIRST data again\n\n");
    
    // thread::sleep(Duration::from_millis(2000));
    {
        let mut started1: std::sync::MutexGuard<'_, bool> = my_mutex1.lock().unwrap();
        
        // dereference operator. access the data (bool) that the MutexGuard is pointing to
        *started1 = true;

        // this seems to be a mechanism to notify the other threads that the value of started has changed
        cvar1.notify_all();
        println!("\nend notify FIRST started= {}\n", started1)
    }
    running1.store(true, Ordering::SeqCst);
    thread::sleep(Duration::from_millis(3000));


}

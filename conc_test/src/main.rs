use inline_colorization::*;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

fn main() {
    // Create a shared state (boolean flag) and a condition variable,
    // wrapped in an Arc for safe sharing across threads
    let pair1: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));

    let running1: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    let running2: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    let running_final: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));

    let pair1_clone: Arc<(Mutex<bool>, Condvar)> = pair1.clone();
    let pair2_clone: Arc<(Mutex<bool>, Condvar)> = pair2.clone();

    let my_running_1: Arc<AtomicBool> = running1.clone();
    let my_running_2: Arc<AtomicBool> = running2.clone();

    let my_running_final: Arc<AtomicBool> = running_final.clone();
    let run_period_ms = 500;
    let loop_speed_ms = 50;
    // make just one super thread
    thread::spawn(move || {
        // Clone the Arc to pass a reference to the shared state to the threads
        let (my_mutex1, my_conditional_variable_1) = &*pair1_clone;
        let (my_mutex2, my_conditional_variable_2) = &*pair2_clone;
        loop {
            let mut i = 0;
            let my_interval = Duration::from_millis(loop_speed_ms);
            println! {"THREAD:{color_red}begin the first loop when signaled{color_reset}"};
            {
                {
                    let mut started1: std::sync::MutexGuard<'_, bool> = my_mutex1.lock().unwrap();
                    *started1 = true;
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
                // println!("THREAD:content 1: is complete");
            }
            my_running_1.store(true, Ordering::SeqCst);
            if !my_running_final.load(Ordering::SeqCst) {
                break;
            }
            println! {"THREAD:{color_red}Begin the second loop when signaled{color_reset}"};
            // second
            {
                i = 0;
                {
                    let mut started2: std::sync::MutexGuard<'_, bool> = my_mutex2.lock().unwrap();
                    *started2 = true;
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
                // println!("THREAD:CONTENT 2: is complete");
            }
            my_running_2.store(true, Ordering::SeqCst);
            if !my_running_final.load(Ordering::SeqCst) {
                break;
            }
        }
        println!("THREAD: FINALLY EXIT");
    });

    fn signal_condition(started: &Mutex<bool>, cvar: &Condvar) -> bool{
        let started_guard: MutexGuard<bool> = started.lock().unwrap();
        
        if *started_guard {
            cvar.notify_all();
            return true
        }
        false
    }

    fn set_running_flag_false(running_flag: Arc<AtomicBool>) {
        running_flag.store(false, Ordering::SeqCst);
    }

    // Loop in the main thread to signal every second
    let (my_mutex1, cvar1) = &*pair1;
    let (my_mutex2, cvar2) = &*pair2;
    // Signal the condition variable
    // When the block is entered, the mutex is locked, and the shared state (started) is
    // accessed and modified

    for _ in 1..=50 {
        // println!("  Notification of 1 to start");
        thread::sleep(Duration::from_millis(5));
        if signal_condition(my_mutex1, cvar1) {
            break;
        }
    }

    // the other threads will not advance even though the notification has been sent
    // until the code block goes out of scope and releases my_mutex
    // the lock guard returned by lock() goes out of scope. This automatically releases
    // the lock on the mutex

    thread::sleep(Duration::from_millis(run_period_ms));
    println!("  ENDING notifications for content 1");

    set_running_flag_false(running1.clone());
    //
    //thread::sleep(Duration::from_millis(3));
    println!("  now kick into the SECOND loop");

    for _ in 1..=50 {
        thread::sleep(Duration::from_millis(5));
        // println!("  Notification of 2 to start");
        if signal_condition(my_mutex2, cvar2){
            break;
        }
    }

    thread::sleep(Duration::from_millis(run_period_ms));

    set_running_flag_false(running2.clone());

    //thread::sleep(Duration::from_millis(1));
    println!("  now kick into the FIRST data again");

    signal_condition(my_mutex1, cvar1);

    for _ in 1..=50 {
        thread::sleep(Duration::from_millis(5));
        if signal_condition(my_mutex1, cvar1) {
            break;
        }
    }
    thread::sleep(Duration::from_millis(run_period_ms));

    set_running_flag_false(running2.clone());
    set_running_flag_false(running1.clone());
    set_running_flag_false(running_final.clone());

    thread::sleep(Duration::from_millis(1000));
    println!("{color_blue}{style_bold}Exit main thread{color_reset}{style_reset}");
}

use inline_colorization::*;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

fn main() {
    // Create a shared state (boolean flag) and a condition variable,
    // wrapped in an Arc for safe sharing across threads

    let running1: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    let running2: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    let running_final: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));

    let my_running_1: Arc<AtomicBool> = running1.clone();
    let my_running_2: Arc<AtomicBool> = running2.clone();
    let my_running_final: Arc<AtomicBool> = running_final.clone();

    let run_period_ms = 500;

    let (sender, receiver) = mpsc::channel();

    // make just one super thread
    thread::spawn(move || {

        loop {
            let mut i = 0;
            println! {"THREAD:{color_red}begin the first loop when signaled{color_reset}"};
            {
                let received_data: (u64, Vec<f64>, String) = receiver.recv().unwrap();
                println!("Received in spawned thread: {}", &received_data.2);
                let loop_rate = Duration::from_millis(received_data.0);

                loop {
                    if !my_running_1.load(Ordering::SeqCst) {
                        break;
                    }
                    i = i + 1;
                    println!("THREAD:content 1:  {}", i);
                    thread::sleep(loop_rate);
                }
                // why doesn't the following line ever run
                println!("THREAD:content 1: is complete");
            }
            my_running_1.store(true, Ordering::SeqCst);
            if !my_running_final.load(Ordering::SeqCst) {
                break;
            }
            println! {"THREAD:{color_red}Begin the second loop when signaled{color_reset}"};
            // second
            {
                i = 0;

                let received_data: (u64, Vec<f64>, String) = receiver.recv().unwrap();
                println!("Received in spawned thread: {}", &received_data.2);
                let loop_rate = Duration::from_millis(received_data.0);

                loop {
                    if !my_running_2.load(Ordering::SeqCst) {
                        break;
                    }
                    i = i + 1;
                    println!("THREAD:CONTENT 2:  {}", i);
                    thread::sleep(loop_rate);
                }
                println!("THREAD:CONTENT 2: is complete");
            }
            my_running_2.store(true, Ordering::SeqCst);
            if !my_running_final.load(Ordering::SeqCst) {
                break;
            }
        }
        println!("THREAD: FINALLY EXIT");
    });

    fn set_running_flag_false(running_flag: Arc<AtomicBool>) {
        running_flag.store(false, Ordering::SeqCst);
    }

    let mut data = (
        20,
        vec![1.0, 2.0, 3.0],
        String::from("Hello, from the main thread!"),
    );

    // the spawned thread is blocked
    println!("sleep time to be sent is {}", data.0);
    sender.send(data).unwrap();
    

    thread::sleep(Duration::from_millis(run_period_ms));
    println!("  ENDING notifications for content 1");

    set_running_flag_false(running1.clone());

    println!("  now kick into the SECOND loop");

    data = (
        50,
        vec![1.0, 2.0, 3.0],
        String::from("Hello, from the main thread!"),
    );

    sender.send(data).unwrap();

    thread::sleep(Duration::from_millis(run_period_ms));

    set_running_flag_false(running2.clone());

    println!("  now kick into the FIRST data again");

    data = (
        10,
        vec![1.0, 2.0, 3.0],
        String::from("Hello 3, from the main thread!"),
    );

    sender.send(data).unwrap();

    thread::sleep(Duration::from_millis(run_period_ms));

    set_running_flag_false(running2.clone());
    set_running_flag_false(running1.clone());
    set_running_flag_false(running_final.clone());

    thread::sleep(Duration::from_millis(1000));
    println!("{color_blue}{style_bold}Exit main thread{color_reset}{style_reset}");
}

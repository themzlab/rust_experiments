use inline_colorization::*;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

const FINAL_MESSAGE: usize = 2;
const FIRST_BUFFER: usize = 0;
const SECOND_BUFFER: usize = 1;

fn main() {
    // Create a shared state (boolean flag) and a condition variable,
    // wrapped in an Arc for safe sharing across threads

    let mut running: Vec<Arc<AtomicBool>> = Vec::new();

    for _ in 0..3 {
        running.push(Arc::new(AtomicBool::new(true)));
    }

    let my_running: Vec<Arc<AtomicBool>> = running.iter().cloned().collect();

    let run_period_ms = 500;

    let (sender, receiver) = mpsc::channel();

    // make just one super thread
    thread::spawn(move || {
        loop {
            let mut i = 0;
            println! {"THREAD:{color_red}begin the first loop when signaled{color_reset}"};
            {
                let received_data: (u64, Vec<f64>, String, usize) = receiver.recv().unwrap();
                println!("Received in spawned thread: {}", &received_data.2);
                let loop_rate = Duration::from_millis(received_data.0);

                loop {
                    if !my_running[FIRST_BUFFER].load(Ordering::Relaxed) {
                        break;
                    }
                    i = i + 1;
                    println!("THREAD:content 1:  {}={}", i, received_data.3);
                    thread::sleep(loop_rate);
                }
                // why doesn't the following line ever run
                println!("THREAD:content 1: is complete");
            }

            my_running[FIRST_BUFFER].store(true, Ordering::Relaxed);

            if !my_running[FINAL_MESSAGE].load(Ordering::Relaxed) {
                // completely exit the thread
                break;
            }
            println! {"THREAD:{color_red}Begin the second loop when signaled{color_reset}"};
            // second
            {
                i = 0;

                let received_data: (u64, Vec<f64>, String, usize) = receiver.recv().unwrap();
                println!("Received in spawned thread: {}", &received_data.2);
                let loop_rate = Duration::from_millis(received_data.0);

                loop {
                    if !my_running[SECOND_BUFFER].load(Ordering::Relaxed) {
                        break;
                    }
                    i = i + 1;
                    println!("THREAD:CONTENT 2:  {}={}", i, received_data.3);
                    thread::sleep(loop_rate);
                }
                println!("THREAD:CONTENT 2: is complete");
            }
            my_running[SECOND_BUFFER].store(true, Ordering::Relaxed);

            if !my_running[FINAL_MESSAGE].load(Ordering::Relaxed) {
                // completely exit the thread
                break;
            }
        }
        println!("THREAD: FINALLY EXIT");
    });

    fn set_running_flag_false(running_flag: Arc<AtomicBool>) {
        running_flag.store(false, Ordering::Relaxed);
    }

    let mut data = (
        10,
        vec![1.0, 2.0, 3.0],
        String::from("Hello, from the main thread!"),
        FIRST_BUFFER,
    );

    // the spawned thread is blocked until data is sent into the queue
    println!("sleep time to be sent is {}", data.0);
    sender.send(data).unwrap();

    thread::sleep(Duration::from_millis(run_period_ms));
    println!("  ENDING notifications for content 1");

    set_running_flag_false(running[FIRST_BUFFER].clone());

    println!("  now kick into the SECOND loop");

    data = (
        50,
        vec![1.0, 2.0, 3.0],
        String::from("Hello, from the main thread!"),
        SECOND_BUFFER,
    );
    sender.send(data).unwrap();
    thread::sleep(Duration::from_millis(run_period_ms));
    set_running_flag_false(running[SECOND_BUFFER].clone());

    println!("  now kick into the FIRST data again");
    data = (
        10,
        vec![1.0, 2.0, 3.0],
        String::from("Hello 3, from the main thread!"),
        FIRST_BUFFER,
    );

    let mut ping_pong = SECOND_BUFFER;
    sender.send(data).unwrap();

    // -----------------------------------------------------------------------------------------

    for _ in 1..=10 {
        // running the current loop
        thread::sleep(Duration::from_millis(run_period_ms));

        // prepare new buffer of data to send which will START
        // have the NEXT data prepared and queue filled before breaking the loop
        data = (
            10,
            vec![1.0, 2.0, 3.0],
            String::from("Hello 3, from the main thread!"),
            ping_pong,
        );
        sender.send(data).unwrap();
        // the loop being shut off is the OPPOSITE one of the data that has been pushed in
        ping_pong = 1 - ping_pong;
        set_running_flag_false(running[ping_pong].clone());
    }

    set_running_flag_false(running[FINAL_MESSAGE].clone());
    set_running_flag_false(running[FIRST_BUFFER].clone());
    set_running_flag_false(running[SECOND_BUFFER].clone());

    thread::sleep(Duration::from_millis(1000));
    println!("{color_blue}{style_bold}Exit main thread{color_reset}{style_reset}");
}

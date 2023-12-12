use pyo3::prelude::*;
use std::thread;
use std::time::Duration;

use inline_colorization::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CHANNEL: (
        Arc<Mutex<mpsc::Sender<(f64, u8)>>>,
        Arc<Mutex<mpsc::Receiver<(f64, u8)>>>
    ) = {
        let (tx, rx) = mpsc::channel();
        (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx)))
    };
    static ref SHARED_BOOL: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref THREAD_STARTED: Mutex<bool> = Mutex::new(false);
    static ref EXIT_REQUEST: AtomicBool = AtomicBool::new(true);
}

#[pyfunction]
fn start_printing_thread() -> PyResult<()> {
    // this is a singleton - it can only be started one time
    let mut started = THREAD_STARTED.lock().unwrap();
    if !*started {
        *started = true;
        let shared_bool_clone = Arc::clone(&SHARED_BOOL);
        let receiver_clone = Arc::clone(&CHANNEL.1);
        thread::spawn(move || {
            //
            loop {
                if !EXIT_REQUEST.load(Ordering::Relaxed) {
                    break;
                }
                {
                    let value = shared_bool_clone.lock().unwrap();
                    println!("SHARED_BOOL value: {}", *value);
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            println!(
                "{style_bold}RUST: loop was cancelled, effectively ending this thread{style_reset}"
            );
            // After exiting the loop
            match receiver_clone.lock().unwrap().recv() {
                Ok(data) => println!(
                    "{style_bold}RUST: Received data from channel: {:?}{style_reset}",
                    data
                ),
                Err(e) => println!(
                    "{style_bold}RUST: Failed to receive data: {}{style_reset}",
                    e
                ),
            }
        });
    }
    Ok(())
}

#[pyfunction]
fn send_value_py(val: (f64, u8)) {
    {
        let tx: std::sync::MutexGuard<'_, mpsc::Sender<(f64, u8)>> = CHANNEL.0.lock().unwrap();
        tx.send(val).unwrap();
    }
}

#[pyfunction]
fn receive_value_py() -> Option<(f64, u8)> {
    let rx: std::sync::MutexGuard<'_, mpsc::Receiver<(f64, u8)>> = CHANNEL.1.lock().unwrap();
    rx.try_recv().ok()
}

#[pyfunction]
fn get_shared_bool() -> PyResult<bool> {
    let shared_bool = SHARED_BOOL.lock().unwrap();
    if *shared_bool {
        println!("{style_bold}RUST: The shared boolean value is true.{style_reset}");
    }
    Ok(*shared_bool)
}

#[pyfunction]
fn set_shared_bool(value: bool) {
    let mut shared_bool = SHARED_BOOL.lock().unwrap();
    *shared_bool = value;
}

#[pyfunction]
fn set_exit_request() {
    // value: bool
    EXIT_REQUEST.store(false, Ordering::SeqCst);
}

#[pyfunction]
fn get_exit_request_status() -> PyResult<bool> {
    Ok(EXIT_REQUEST.load(Ordering::SeqCst))
}

#[pymodule]
fn testchannels(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(send_value_py, m)?)?;
    m.add_function(wrap_pyfunction!(receive_value_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_shared_bool, m)?)?;
    m.add_function(wrap_pyfunction!(set_shared_bool, m)?)?;
    m.add_function(wrap_pyfunction!(start_printing_thread, m)?)?;
    m.add_function(wrap_pyfunction!(set_exit_request, m)?)?;
    m.add_function(wrap_pyfunction!(get_exit_request_status, m)?)?;

    Ok(())
}

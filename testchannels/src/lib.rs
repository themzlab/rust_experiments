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
        Arc<Mutex<mpsc::Sender<(f64, u8, Vec<f64>)>>>,
        Arc<Mutex<mpsc::Receiver<(f64, u8, Vec<f64>)>>>
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
            let mut myfloat: f64 = 0.0;
            let mut myinteger: u8 = 0;
            //let mut local_vec: Vec<f64> = Vec::new();
            let mut local_vec: Vec<f64> = vec![0.0; 3];
            // let mut local_vec: Vec<f64> = vec![0.0; 74];

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
                // Ok(data) => {
                //     _f = Some(data.0);
                //     _i = Some(data.1);
                Ok((_myfloat, _myinteger, _vec)) => {
                    myfloat = _myfloat;
                    myinteger = _myinteger;
                    if _vec.len() == 3 {
                        println!("correct length");
                    }
                    local_vec = _vec;
                    // _f and _i are the float and integer values from the tuple, respectively
                    // You can add any mathematical operations here using _f and _i
                }
                Err(e) => println!("Failed to receive data: {}", e),
            }
            for (index, value) in local_vec.iter().enumerate() {
                println!("RUST: Element at index {}: {}", index, value);
            }
            println!(
                "{style_bold}RUST: my float was {} and integer {} {style_reset}",
                myfloat, myinteger
            );
        });
    }
    Ok(())
}

#[pyfunction]
fn send_value_py(val: (f64, u8, Vec<f64>)) {
    {
        let tx: std::sync::MutexGuard<'_, mpsc::Sender<(f64, u8, Vec<f64>)>> =
            CHANNEL.0.lock().unwrap();
        tx.send(val).unwrap();
    }
}

#[pyfunction]
fn receive_value_py() -> Option<(f64, u8, Vec<f64>)> {
    let rx: std::sync::MutexGuard<'_, mpsc::Receiver<(f64, u8, Vec<f64>)>> =
        CHANNEL.1.lock().unwrap();
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

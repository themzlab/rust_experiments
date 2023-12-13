use pyo3::prelude::*;
use std::thread;
use std::time::{Duration, Instant};

use inline_colorization::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CHANNEL: (
        Arc<Mutex<mpsc::Sender<(u64, u8, Vec<f64>)>>>,
        Arc<Mutex<mpsc::Receiver<(u64, u8, Vec<f64>)>>>
    ) = {
        let (tx, rx) = mpsc::channel();
        (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx)))
    };
    static ref ERROR_CODE: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
    static ref SHARED_BOOL: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref THREAD_IS_STARTED: Mutex<bool> = Mutex::new(false);
    static ref TRIGGER_STATE_CHANGE: AtomicBool = AtomicBool::new(false);
    static ref EXIT_REQUEST: AtomicBool = AtomicBool::new(false);
}

#[pyfunction]
fn initialize_module() -> PyResult<()> {
    // this is a singleton - it can only be started one time
    let mut started = THREAD_IS_STARTED.lock().unwrap();
    if !*started {
        *started = true;
        let _shared_bool_clone = Arc::clone(&SHARED_BOOL);
        let receiver_clone = Arc::clone(&CHANNEL.1);
        thread::spawn(move || {
            //
            let mut loop_micros: u64 = 20000;
            let mut program_state: u8 = 0;
            //let mut local_vec: Vec<f64> = Vec::new();
            let mut local_vec: Vec<f64> = vec![0.0; 3];
            // let mut local_vec: Vec<f64> = vec![0.0; 74];
            loop {
                // to enable multiple entries of the commands

                // recieve data so we know what parameters to run
                // let received_data: (u64, u8, Vec<f64>) = receiver_clone.lock().unwrap().recv();
                // loop_micros = received_data.0;
                // program_state = received_data.1;
                // local_vec = received_data.2;

                // ========================================================================================== BLOCKS
                if let Ok((loop_speed_from_python, state_requested, _vec)) =
                    receiver_clone.lock().unwrap().recv()
                {
                    loop_micros = loop_speed_from_python;
                    program_state = state_requested;
                    if _vec.len() == 3 {
                        println!("correct length");
                    }
                    local_vec = _vec;
                    TRIGGER_STATE_CHANGE.store(false, Ordering::Relaxed);
                } else {
                    // I don't know how to reach this because when I send data from Python that is bad
                    // an exception is thrown in Python
                    set_error_code(4);
                }

                let loop_speed = Duration::from_micros(loop_micros);

                // ===================================================================== BLOCKS UNTIL LOOP BREAK IS SENT
                {
                    for duty_cycle in local_vec.iter().cycle() {
                        let start = Instant::now();
                        if TRIGGER_STATE_CHANGE.load(Ordering::Relaxed) {
                            break;
                        }
                        println!("duty cycle: {}", *duty_cycle);
                        let elapsed = start.elapsed();
                        if let Some(sleep_duration) = loop_speed.checked_sub(elapsed) {
                            // only sleep if there is a positive time duration after subtracting elapsed time
                            // from the desired interval
                            std::thread::sleep(sleep_duration);
                        }
                    }
                }
                println!("{style_bold}RUST: loop was cancelled, effectively ending this thread{style_reset}");
                // After exiting the loop
                if EXIT_REQUEST.load(Ordering::Relaxed) {
                    break;
                }

                for (index, value) in local_vec.iter().enumerate() {
                    println!("RUST: Element at index {}: {}", index, value);
                }
                println!(
                    "{style_bold}RUST: my loop speed was {}us and state {} {style_reset}",
                    loop_micros, program_state
                );
                if EXIT_REQUEST.load(Ordering::Relaxed) {
                    break;
                }
                //
            } // outer loop, exit one time only
        });
    }
    Ok(())
}

// This function can be used inside any thread to update the value
fn set_error_code(value: u8) {
    let mut shared_u8 = ERROR_CODE.lock().unwrap();
    *shared_u8 = value;
}

#[pyfunction]
fn get_error_code() -> PyResult<u8> {
    let shared_u8 = ERROR_CODE.lock().unwrap();
    Ok(*shared_u8)
}

#[pyfunction]
fn clear_error_code() {
    let mut _error_code = ERROR_CODE.lock().unwrap();
    *_error_code = 0;
}

#[pyfunction]
fn send_value_py(val: (u64, u8, Vec<f64>)) {
    TRIGGER_STATE_CHANGE.store(true, Ordering::SeqCst);
    {
        let tx: std::sync::MutexGuard<'_, mpsc::Sender<(u64, u8, Vec<f64>)>> =
            CHANNEL.0.lock().unwrap();
        tx.send(val).unwrap();
    }
}

#[pyfunction]
fn receive_value_py() -> Option<(u64, u8, Vec<f64>)> {
    let rx: std::sync::MutexGuard<'_, mpsc::Receiver<(u64, u8, Vec<f64>)>> =
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
fn set_exit_request(value: bool) {
    // to make sure we drop out of the outer loop
    EXIT_REQUEST.store(value, Ordering::SeqCst);

    // to make sure we drop out of the inner loop, but only after making sure the outer one will exit
    TRIGGER_STATE_CHANGE.store(value, Ordering::SeqCst);
}

#[pyfunction]
fn trigger_state_change() {
    // should store values before triggering the state change
    TRIGGER_STATE_CHANGE.store(true, Ordering::SeqCst);
}

#[pyfunction]
fn get_exit_request_status() -> PyResult<bool> {
    // this will normally be false.  after it has been set to true the thread will exit
    // and
    Ok(EXIT_REQUEST.load(Ordering::SeqCst))
}

#[pyfunction]
fn is_module_started() -> PyResult<bool> {
    // checks if the spawned thread has started running
    let started = THREAD_IS_STARTED.lock().unwrap();
    Ok(*started)
}

#[pyfunction]
fn is_module_started_and_active() -> PyResult<bool> {
    // checks if the spawned thread has started running
    let started = THREAD_IS_STARTED.lock().unwrap();
    Ok(*started && !EXIT_REQUEST.load(Ordering::SeqCst))
}

#[pymodule]
fn testchannels(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(send_value_py, m)?)?;
    m.add_function(wrap_pyfunction!(receive_value_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_shared_bool, m)?)?;
    m.add_function(wrap_pyfunction!(set_shared_bool, m)?)?;
    m.add_function(wrap_pyfunction!(initialize_module, m)?)?;
    m.add_function(wrap_pyfunction!(set_exit_request, m)?)?;
    m.add_function(wrap_pyfunction!(trigger_state_change, m)?)?;
    m.add_function(wrap_pyfunction!(get_exit_request_status, m)?)?;
    m.add_function(wrap_pyfunction!(is_module_started, m)?)?;
    m.add_function(wrap_pyfunction!(is_module_started_and_active, m)?)?;
    m.add_function(wrap_pyfunction!(get_error_code, m)?)?;
    m.add_function(wrap_pyfunction!(clear_error_code, m)?)?;
    Ok(())
}

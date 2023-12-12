use pyo3::prelude::*;
use std::sync::{mpsc, Arc, Mutex};
use inline_colorization::*;

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


#[pymodule]
fn testchannels(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(send_value_py, m)?)?;
    m.add_function(wrap_pyfunction!(receive_value_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_shared_bool, m)?)?;
    m.add_function(wrap_pyfunction!(set_shared_bool, m)?)?;
    Ok(())
}

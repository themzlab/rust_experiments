use pyo3::prelude::*;
use std::sync::{mpsc, Arc, Mutex};
#[macro_use]
extern crate lazy_static;


lazy_static! {
    static ref CHANNEL: (Arc<Mutex<mpsc::Sender<(f64, i32)>>>, Arc<Mutex<mpsc::Receiver<(f64, i32)>>>) = {
        let (tx, rx) = mpsc::channel();
        (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx)))
    };
}



#[pyfunction]
fn send_value_py(val: (f64, i32)) {
    let tx = CHANNEL.0.lock().unwrap();
    tx.send(val).unwrap();
}



#[pyfunction]
fn receive_value_py() -> Option<(f64, i32)> {
    let rx = CHANNEL.1.lock().unwrap();
    rx.try_recv().ok()
}



#[pymodule]
fn testchannels(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(send_value_py, m)?)?;
    m.add_function(wrap_pyfunction!(receive_value_py, m)?)?;
    Ok(())
}

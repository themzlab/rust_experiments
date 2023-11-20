use pyo3::prelude::*;
use rppal::pwm::{Channel, Polarity, Pwm};
use std::{sync::Arc, thread, time::Duration, sync::atomic::{AtomicBool, Ordering}};
use std::thread::JoinHandle;

static mut RUNNING: Option<Arc<AtomicBool>> = None;
static mut PWM_THREAD: Option<JoinHandle<()>> = None;

#[pyfunction]
fn start_pwm(duty_cycles: Vec<f64>, sleep_ms: u64) -> PyResult<()> {
    let running = Arc::new(AtomicBool::new(true));
    unsafe {
        RUNNING = Some(running.clone());
    }

    let handle = thread::spawn(move || {
        let pwm = Pwm::with_frequency(Channel::Pwm0, 20_000.0, 0.0, Polarity::Normal, true).unwrap();

        for duty_cycle in duty_cycles.iter().cycle() {
            if !running.load(Ordering::SeqCst) {
                break;
            }
            pwm.set_duty_cycle(*duty_cycle).unwrap();
            thread::sleep(Duration::from_millis(sleep_ms));
        }
    });

    unsafe {
        PWM_THREAD = Some(handle);
    }

    Ok(())
}

#[pyfunction]
fn stop_pwm() -> PyResult<()> {
    unsafe {
        if let Some(running) = &RUNNING {
            running.store(false, Ordering::SeqCst);
            if let Some(handle) = PWM_THREAD.take() {
                let _ = handle.join();
            }
        }
    }
    Ok(())
}

#[pymodule]
fn pwm_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_pwm, m)?)?;
    m.add_function(wrap_pyfunction!(stop_pwm, m)?)?;
    Ok(())
}


use pyo3::prelude::*;
use rppal::pwm::{Channel, Polarity, Pwm};
use std::{thread, time::Duration};

#[pyfunction]
fn start_pwm(duty_cycles: Vec<f64>) -> PyResult<()> {
    let pwm = Pwm::with_frequency(Channel::Pwm0, 20_000.0, 0.0, Polarity::Normal, true).unwrap();

    for duty_cycle in duty_cycles.iter().cycle() {
        pwm.set_duty_cycle(*duty_cycle).unwrap();
        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}

#[pymodule]
fn pwm_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_pwm, m)?)?;
    Ok(())
}

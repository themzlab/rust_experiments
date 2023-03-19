use pyo3::prelude::*;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use rppal::spi::{Bus, Mode, Segment, SlaveSelect, Spi};

// use pyo3::{wrap_pyfunction, };


#[pyclass]
struct AdcModule {
    // #[pyo3(get)]
    name: String,
}


#[pymethods]
impl AdcModule {
    #[new]
    fn new(name: String) -> Self {
        println!("Thread {:} has been started", name);
        let _t: JoinHandle<Self> = thread::spawn(move || loop {
            println!("Hello from Rust thread!");
            thread::sleep(Duration::from_secs(1));
        });

        AdcModule {name}
    }

    fn test(&self) {
        // self.name = "hi mark".to_string();
        println!("{:}", self.name);
    }
}

// the name below must match the project name
#[pymodule]
fn adc_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AdcModule>()?;
    Ok(())
}

use pyo3::prelude::*;
use std::thread;
use std::time::Duration;
// use pyo3::{wrap_pyfunction, };


#[pyclass]
struct TestThreads {
    // #[pyo3(get)]
    name: String,
}


#[pymethods]
impl TestThreads {
    #[new]
    fn new(name: String) -> Self {
        println!("Thread {:} has been started", name);
        let _t = thread::spawn(move || loop {
            println!("Hello from Rust thread!");
            thread::sleep(Duration::from_secs(1));
        });

        TestThreads {name}
    }
}

// the name below must match the project name
#[pymodule]
fn test_threads(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TestThreads>()?;
    Ok(())
}

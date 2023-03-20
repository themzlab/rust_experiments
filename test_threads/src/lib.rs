use pyo3::prelude::*;
use std::thread::{self, JoinHandle};
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
        let my_name = name.to_string();
        // create the Thread, also pass ownership of my_name into it
        let _t: JoinHandle<Self> = thread::spawn(move || {
            println!{"started new thread named {:}", my_name};
            // do other initialization I guess
            let mut counter: i32 = 0;
            loop {
                counter = counter + 1;
                println!("Hello from {:},a Rust thread! loop {:}", my_name, counter);
                thread::sleep(Duration::from_secs(2));
            }
        });

        TestThreads {name}
    }

    fn begin_thread(&self) {
        println!("MY beginning thread{:}", self.name);
        let my_name = self.name.to_string();
        // create the Thread, also pass ownership of my_name into it
        let _t: JoinHandle<Self> = thread::spawn(move || {
            println!{"LATER, started new thread named {:}", my_name};
            // do other initialization I guess
            let mut counter: i32 = 0;
            loop {
                counter = counter + 1;
                println!("---->{:},a Rust thread! loop {:}", my_name, counter);
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    fn test(&self) {
        // self.name = "hi mark".to_string();
        println!("{:}", self.name);
    }
}

// the name below must match the project name
#[pymodule]
fn test_threads(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TestThreads>()?;
    Ok(())
}

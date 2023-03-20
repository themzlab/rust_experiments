use pyo3::prelude::*;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use rppal::spi::{Bus, Mode, Segment, SlaveSelect, Spi};
use std::error::Error;
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

        let _t: JoinHandle<Self> = thread::spawn(move || {
            eprintln!("created the spi interface");
    
            loop {
                println!("Hello from Rust thread!");
                thread::sleep(Duration::from_secs(1));
            }
        }
        );

        AdcModule {name}
    }

    // fn begin_thread(&self) -> Result<(), Box<dyn Error>> {
    //     println!("MY beginning thread{:}", self.name);
    //     let my_name = self.name.to_string();
    //     let spi: Spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 100_000, Mode::Mode0)?;
    //     // create the Thread, also pass ownership of my_name into it
    //     let _t: JoinHandle<Self> = thread::spawn(move || {
    //         println!{"LATER, started new thread named {:}", my_name};
    //         // do other initialization I guess
    //         let mut counter: i32 = 0;
    //         let mut read_buffer: [u8; 2] = [0u8; 2]; // the spi read will be according to the size of the buffer
    //         loop {
    //             spi.transfer_segments(&[
    //                 // Segment::with_write(&[READ, 0, 0, 0]),
    //                 Segment::with_read(&mut read_buffer),
    //             ]);
    //             counter = counter + 1;
    //             println!("---->{:},a Rust thread! loop {:}", my_name, counter);
    //             thread::sleep(Duration::from_secs(1));
    //         }
    //     });
    //     Ok(())
    // }

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

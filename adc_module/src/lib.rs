use pyo3::prelude::*;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use rppal::spi::{Bus, Mode, Segment, SlaveSelect, Spi};
// use std::error::Error;
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
                thread::sleep(Duration::from_secs(5));
            }
        }
        );

        AdcModule {name}
    }

    fn begin_thread(&self) {
        println!("MY beginning thread{:}", self.name);
        let my_name: String = self.name.to_string();
        let spi: Spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 100_000, Mode::Mode0).unwrap();

        // create the Thread, also pass ownership of my_name into it
        let _t: JoinHandle<Self> = thread::spawn(move || {
            const OFFSET_ADC: i32 = 0;
            const DIVISOR_ADC: f32 = 100.0;
            println!{"LATER, started new thread named {:}", my_name};
            // do other initialization I guess
            let mut counter: i32 = 0;
            let mut read_buffer: [u8; 2] = [0u8; 2]; // the spi read will be according to the size of the buffer
            loop {
                spi.transfer_segments(&[
                    Segment::with_read(&mut read_buffer),
                ]).unwrap();
                let _output = u16::from_be_bytes(read_buffer);
                let mut output = _output as i32;
                output = output-OFFSET_ADC;
                let mut torque =output as f32;
                torque = torque / DIVISOR_ADC;
                //
                counter = counter + 1;
                println!("---->{:},a Rust thread! loop {:} value {:.3}", my_name, counter, torque);
                thread::sleep(Duration::from_secs(1));
            }
        });
        
    }

    fn begin_reading(&self) {
        println!("begin_reading{:}", self.name);
        let my_name = self.name.to_string();
        // create the Thread, also pass ownership of my_name into it
        let _t: JoinHandle<Self> = thread::spawn(move || {
            println!{"inside the new thread, begin_reading {:}", my_name};
            // do other initialization I guess
            let mut counter: i32 = 0;
            loop {
                counter = counter + 1;
                println!("---->{:},a Test thread; begin_reading {:}", my_name, counter);
                thread::sleep(Duration::from_millis(500));
            }
        });
    }

    fn test(&self) {
        println!("{:}", self.name);
    }
}

// the name below must match the project name
#[pymodule]
fn adc_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AdcModule>()?;
    Ok(())
}

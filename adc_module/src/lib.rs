use pyo3::prelude::*;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use rppal::spi::{Bus, Mode, Segment, SlaveSelect, Spi};
use std::sync::mpsc;
use std::sync::mpsc::{Sender}; // , Receiver};

// use std::error::Error;
// use pyo3::{wrap_pyfunction, };


#[pyclass]
struct AdcModule {
    #[pyo3(get)]
    name: String,
    //
    offset_adc: i32,
    divisor_adc: f32,
    tx: Sender<i32>, // could not keep rx here because it wants to be passed to the Thread
}


#[pymethods]
impl AdcModule {
    #[new]
    fn new(name: String, _offset_adc: i32, _divisor_adc: f32) -> Self {
        eprintln!("Thread {:} has been started", name);

        let (tx, rx) = mpsc::channel();

        let spi: Spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 100_000, Mode::Mode0).unwrap();
        let divisor_adc: f32 = _divisor_adc;
        let offset_adc: i32 = _offset_adc;
        let my_name: String = name.to_string();

        let _t: JoinHandle<Self> = thread::spawn(move || {
    
            eprintln!{"LATER, started new thread named {:}", my_name};
            // do other initialization I guess
            let mut counter: i32 = 0;
            let mut read_buffer: [u8; 2] = [0u8; 2]; // the spi read will be according to the size of the buffer
            loop {
                
                // recieve new commands into this running Thread
                if let Ok(received) = rx.try_recv() {
                    println!("Got command: {}", received);
                }

                spi.transfer_segments(&[
                    Segment::with_read(&mut read_buffer),
                ]).unwrap();

                let _output = u16::from_be_bytes(read_buffer);
                let mut output = _output as i32;
                output -= _offset_adc;
                let mut torque =output as f32;
                torque /= _divisor_adc;
                //
                counter += 1;
                eprintln!("---->{:},a Rust thread! loop {:} value {:.3}", my_name, counter, torque);
                thread::sleep(Duration::from_secs(1));
            }
        });

        AdcModule {name, offset_adc, divisor_adc, tx}
    }

    fn begin_thread(&self) {
        eprintln!("MY beginning thread{:} divisor {:.3}", self.name, self.divisor_adc);
        let divisor_adc: f32 = self.divisor_adc;
        let offset_adc: i32 = self.offset_adc;
        
    }

    fn begin_reading(&self) {
        eprintln!("begin_reading{:}", self.name);
        let my_name = self.name.to_string();
        // create the Thread, also pass ownership of my_name into it
        let _t: JoinHandle<Self> = thread::spawn(move || {
            eprintln!{"inside the new thread, begin_reading {:}", my_name};
            
            let mut counter: i32 = 0;
            loop {
                counter += 1;
                eprintln!("---->{:},a Test thread; begin_reading {:}", my_name, counter);
                thread::sleep(Duration::from_millis(1000));
            }
        });
    }

    fn test(&self, value_to_send: i32) {
        eprintln!("-------> {:} sending task value {:}", self.name, value_to_send);
        self.tx.send(value_to_send).unwrap();
    }

}

// the name below must match the project name
#[pymodule]
fn adc_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AdcModule>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__doc__", "https://github.com/themzlab/rust_experiments/tree/main/adc_module")?;
    Ok(())
}

// test_adc/src/main.rs 
// https://github.com/golemparts/rppal#spi
use std::error::Error;
use rppal::spi::{Bus, Mode, Segment, SlaveSelect, Spi};
use std::{thread, time};
use time::{Duration, };

fn main() -> Result<(), Box<dyn Error>> {

    const OFFSET_ADC: i32 = 0;
    const DIVISOR_ADC: f32 = 100.0;
    
    let spi: Spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 100_000, Mode::Mode0)?;
    eprintln!("created the spi interface");

    // at 100_000 spi speed, reading 4 bytes, 3910  us sleep gives 200SPS if not doing anythingwith this data
    let sleep_time: Duration = time::Duration::from_micros(4300); // 3910 for reading 3 bytes
    let mut read_buffer: [u8; 2] = [0u8; 2]; // the spi read will be according to the size of the buffer

    loop {
        spi.transfer_segments(&[
            // Segment::with_write(&[READ, 0, 0, 0]),
            Segment::with_read(&mut read_buffer),
        ])?;
        // 98 us between reads if there is NOTHING to do but loop
        // reading 4 bytes at this speed takes 742 us
        // reading 3 bytes takes 546 us
        // 
        thread::sleep(sleep_time);

        let _output = u16::from_be_bytes(read_buffer);
        let mut output = _output as i32;
        output = output-OFFSET_ADC;
        let mut torque =output as f32;
        torque = torque / DIVISOR_ADC;
        //
        println!("{:.3}", torque);
    }
    // Ok(())
}

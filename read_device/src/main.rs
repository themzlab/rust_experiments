use std::io::{self, Read};
use std::time::{Duration, Instant};
use std::fs::OpenOptions;
use std::fs::File;
use std::os::unix::fs::OpenOptionsExt;
use libc::O_DIRECT; // Import O_DIRECT constant from libc

// Rest of your code to read and parse the value


fn main() -> io::Result<()> {
    // Path to the ADC sysfs entry
    // let adc_path =
    //     "/sys/devices/platform/soc/fe804000.i2c/i2c-1/1-0029/iio:device0/in_intensity_ir_raw";

    // let device_path: &str = "/sys/devices/platform/soc/fe804000.i2c/i2c-1/1-0029/iio:device0/";
    let device_path: &str = "/home/pi/";
    let property_name: &str = "in_illuminance_input";

    // Open the file for reading
    // let mut file = File::open(adc_path)?;
    // let mut file: File = File::open(format!("{device_path}{property_name}"))?;
    // let mut options: OpenOptions = OpenOptions::new();
    // let mut file = options.read(true).open(format!("{device_path}{property_name}")).unwrap();

    // Open the file with O_DIRECT flag to disable caching
    // let mut file = OpenOptions::new().read(true).custom_flags(libc::O_DIRECT).open(format!("{device_path}{property_name}"))?;

    // Number of samples to read per second
    let samples_per_second: i32 = 1;

    // Calculate the delay between samples in microseconds
    let sample_delay: Duration = Duration::from_micros(1_000_000 / samples_per_second as u64);

    let mut adc_value_str: String = String::new();
    let mut count: i32 = 0;
    
    loop {
        let fullpathname = format!("{device_path}{property_name}"); 
        let start: Instant = Instant::now();
        let mut file: File = File::open(fullpathname)?;
        file.read_to_string(&mut adc_value_str)?;
        //file.sync_all()?;

        // Parse the ADC value to a u32
        if let Ok(adc_value) = adc_value_str.trim().parse::<u32>() {
            // Print the ADC value
            println!("ADC Value: {} count {}", adc_value, count);
            adc_value_str = "".to_string();
        } else {
            eprintln!("Failed to parse ADC value: {} count{}", adc_value_str, count);
        }

        let elapsed: Duration = start.elapsed();
        count +=1;
        if let Some(sleep_duration) = sample_delay.checked_sub(elapsed) {
            // only sleep if there is a positive time duration after subtracting elapsed time
            // from the desired interval
            std::thread::sleep(sleep_duration);
        }
    }
}
